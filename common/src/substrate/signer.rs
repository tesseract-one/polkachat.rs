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
    ext::frame_metadata::v14::{ExtrinsicMetadata, SignedExtensionMetadata},
    tx::Signer,
    utils::AccountId32,
    Metadata
};
use parity_scale_codec::Encode;
use scale_info::{form::PortableForm, PortableRegistry};
use subxt_signer::sr25519;

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
        let public = sr25519::PublicKey(
            account.public_key.clone().try_into().map_err(|_| crate::Error::PublicKey)?
        );
        Ok(Self {
            tesseract: tesseract,
            account: public.to_account_id(),
            path: account.path.clone(),
            metadata,
        })
    }

    fn get_medatada_info(&self) -> (ExtrinsicMetadata<PortableForm>, PortableRegistry) {
        let ext_meta = self.metadata.extrinsic();

        let extensions = ext_meta.signed_extensions().iter().map(|ext| {
            SignedExtensionMetadata {
                identifier: ext.identifier().into(),
                ty: ext.extra_ty().into(),
                additional_signed: ext.additional_ty().into()
            }
        }).collect();

        let meta = ExtrinsicMetadata {
            ty: ext_meta.call_ty().into(),
            version: ext_meta.version(),
            signed_extensions: extensions
         };

         (meta, self.metadata.types().clone())
    }
}

impl Signer<subxt::PolkadotConfig> for TesseractSigner {
    /// Return the "from" account ID.
    fn account_id(&self) -> <subxt::PolkadotConfig as subxt::Config>::AccountId {
        self.account.clone()
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
        let (meta, registry) = self.get_medatada_info();
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
        let signature = sr25519::Signature(bytes.try_into().unwrap());
        signature.into()
    }
}
