//===-------------- signer.rs --------------------------------------------===//
//  Copyright 2023, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::sync::Arc;

use futures::executor;

use tesseract::client::Service;
use tesseract_protocol_substrate::{AccountType, GetAccountResponse, Substrate, SubstrateService};

use subxt::{
    ext::{
        codec::Encode,
        frame_metadata::v14::ExtrinsicMetadata,
        scale_value::scale::PortableRegistry,
        sp_core::sr25519,
        sp_runtime::{
            scale_info::form::PortableForm,
            traits::IdentifyAccount,
            AccountId32, MultiSigner
        },
    },
    tx::Signer,
    Metadata
};

pub struct TesseractSigner {
    tesseract: Arc<dyn Service<Protocol = Substrate>>,
    metadata: Metadata,
    account: AccountId32,
    path: String,
}

impl TesseractSigner {
    pub (crate) fn new(
        tesseract: Arc<dyn Service<Protocol = Substrate>>,
        account: &GetAccountResponse,
        metadata: Metadata,
    ) -> crate::Result<Self> {
        let pk: sr25519::Public = account.public_key.as_slice().try_into().map_err(|_| { //error is () here anyway
            crate::Error::PublicKey
        })?;
        let public: MultiSigner = pk.into();
        let account_id = public.clone().into_account();
        Ok(Self {
            tesseract: tesseract,
            account: account_id,
            path: account.path.clone(),
            metadata,
        })
    }

    fn get_medatada_info(
        &self,
        extrinsic_data: &[u8],
    ) -> Result<(ExtrinsicMetadata<PortableForm>, PortableRegistry), subxt::error::Error> {
        let pallet_idx = extrinsic_data[0];
        let pallet = self
            .metadata
            .runtime_metadata()
            .pallets
            .iter()
            .find(|p| p.index == pallet_idx)
            .ok_or("Pallet not found!")?;
        let call_ty_id = pallet.calls.as_ref().ok_or("Pallet doesn't have calls")?.ty;
        let mut meta = self.metadata.runtime_metadata().extrinsic.clone();
        meta.ty = call_ty_id.into();
        Ok((meta, self.metadata.types().clone()))
    }
}

impl Signer<subxt::PolkadotConfig> for TesseractSigner {
    /// Return the "from" account ID.
    fn account_id(&self) -> &<subxt::PolkadotConfig as subxt::Config>::AccountId {
        &self.account
    }

    /// Return the "from" address.
    fn address(&self) -> <subxt::PolkadotConfig as subxt::Config>::Address {
        self.account.clone().into()
    }

    /// Takes a signer payload for an extrinsic, and returns a signature based on it.
    ///
    /// Some signers may fail, for instance because the hardware on which the keys are located has
    /// refused the operation.
    fn sign(&self, signer_payload: &[u8]) -> <subxt::PolkadotConfig as subxt::Config>::Signature {
        let (meta, registry) = self.get_medatada_info(signer_payload).unwrap();
        let extrinsic_metadata = meta.encode();
        let extrinsic_types = registry.encode();
        let signed_future = Arc::clone(&self.tesseract).sign_transaction(
            AccountType::Sr25519,
            &self.path,
            signer_payload,
            &extrinsic_metadata,
            &extrinsic_types,
        );

        let result = executor::block_on(signed_future).expect("signing failed");
        let bytes: &[u8] = result.as_ref();
        let signature: sr25519::Signature = bytes.try_into().unwrap();
        signature.into()
    }
}
