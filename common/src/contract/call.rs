use scale_decode::{DecodeAsType, DecodeAsFields};
use scale_encode::EncodeAsType;
use subxt::{
    events::StaticEvent, 
    ext::codec::{Compact, Encode, Decode},
    tx::TxPayload, tx::Payload,
    Error, Metadata
};

use super::primitives::{ContractExecResult, Weight, DecodeWithMetadata};

#[derive(DecodeAsType, Clone, Debug)]
pub struct ContractEmittedEvent<AccountId: DecodeAsType> {
    pub contract: AccountId,
    pub data: Vec<u8>,
}

impl<T: DecodeAsType> StaticEvent for ContractEmittedEvent<T> where Self: DecodeAsFields {
    const PALLET: &'static str = "Contracts";
    const EVENT: &'static str = "ContractEmitted";
}

impl<T: DecodeAsType> ContractEmittedEvent<T> {
    pub fn try_parse_event<E: Decode>(&self) -> Result<E, Error> {
        Ok(E::decode(&mut self.data.as_ref())?)
    }
}

pub trait StaticCall {
    /// Pallet name.
    const PALLET: &'static str;
    /// Call name.
    const CALL: &'static str;
}

#[derive(EncodeAsType, Clone)]
pub struct ContractCallCall<Address: EncodeAsType> {
    dest: Address,
    #[codec(compact)]
    value: u128,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<u128>>,
    data: Vec<u8>,
}

impl<Address: EncodeAsType> ContractCallCall<Address> {
    pub fn new(
        id: Address,
        value: u128,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            dest: id,
            value,
            gas_limit,
            storage_deposit_limit: storage_deposit_limit.map(|v| v.into()),
            data,
        }
    }

    pub fn new_call(
        id: Address,
        value: u128,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
        method: [u8; 4],
    ) -> Self {
        Self::new(
            id,
            value,
            gas_limit,
            storage_deposit_limit,
            method.into(),
        )
    }

    pub fn add_parameter<P: Encode>(mut self, param: P) -> Self {
        param.encode_to(&mut self.data);
        self
    }

    pub fn tx(self) -> impl TxPayload {
        Payload::<Self>::new(Self::PALLET, Self::CALL, self)
    }
}

impl<T: EncodeAsType> StaticCall for ContractCallCall<T> {
    /// Pallet name.
    const PALLET: &'static str = "Contracts";
    /// Call name.
    const CALL: &'static str = "call";
}

#[derive(Encode, Clone)]
pub struct ContractCallQuery<AccountId: Encode> {
    origin: AccountId,
    dest: AccountId,
    value: u128,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<u128>,
    input_data: Vec<u8>,
}

impl<AccountId: Encode> ContractCallQuery<AccountId> {
    pub fn new_call(
        contract: AccountId,
        from: AccountId,
        value: u128,
        gas_limit: Option<Weight>,
        storage_deposit_limit: Option<u128>,
        method: [u8; 4],
    ) -> Self {
        Self {
            origin: from,
            dest: contract,
            value,
            gas_limit,
            storage_deposit_limit,
            input_data: method.into(),
        }
    }

    pub fn add_parameter<P: Encode>(mut self, param: P) -> Self {
        param.encode_to(&mut self.input_data);
        self
    }

    pub fn as_param(&self) -> Vec<u8> {
        self.encode()
    }
}

pub fn parse_query_result<T: Decode>(data: Vec<u8>, meta: &Metadata) -> Result<(T, Weight), Error> {
    let result = ContractExecResult::<u128>::decode_with_metadata(&mut data.as_ref(), meta)?;
    let rresult: Result<_, _> = result.result.into();
    let res_data = rresult?.data;
    Ok((T::decode(&mut res_data.as_ref())?, result.gas_required))
}
