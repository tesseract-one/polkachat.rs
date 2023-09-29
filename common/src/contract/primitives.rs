use parity_scale_codec::*;
use std::fmt::Debug;
use scale_encode::EncodeAsType;
use subxt::{ext::scale_value::scale::{decode_as_type, encode_as_type}, error::MetadataError};
use subxt::error::DispatchError;
use subxt::Metadata;

// This is a utility types which copied from pallet_contract_primitives and sp_runtime crates
// sp_runtime can't be built on iOS, so we forced to do this

#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, EncodeAsType, Debug)]
pub struct Weight {
	#[codec(compact)]
	ref_time: u64,
	#[codec(compact)]
	proof_size: u64,
}

impl Weight {
    /// Construct [`Weight`] from weight parts, namely reference time and proof size weights.
    pub const fn from_parts(ref_time: u64, proof_size: u64) -> Self {
        Self { ref_time, proof_size }
    }
}

#[derive(Clone, Debug)]
pub struct ContractResult<R: Debug, Balance: Debug> {
	pub gas_consumed: Weight,
	pub gas_required: Weight,
	pub storage_deposit: StorageDeposit<Balance>,
	pub debug_message: Vec<u8>,
	pub result: R
}

impl<R: DecodeWithMetadata + Debug, B: Decode + Debug> DecodeWithMetadata for ContractResult<R, B> {
	fn decode_with_metadata(input: &mut &[u8], metadata: &Metadata) -> Result<Self, subxt::Error> {
		let gas_consumed = Weight::decode(input)?;
		let gas_required = Weight::decode(input)?;
		let storage_deposit = StorageDeposit::<B>::decode(input)?;
		let debug_message = Vec::<u8>::decode(input)?;
		let result = R::decode_with_metadata(input, metadata)?;
		Ok(Self { gas_consumed, gas_required, storage_deposit, debug_message, result })
	}
}

pub type ContractExecResult<Balance> =
	ContractResult<DispatchResult<ExecReturnValue>, Balance>;


#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, Debug)]
pub struct ReturnFlags(u32);

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct ExecReturnValue {
	pub flags: ReturnFlags,
	pub data: Vec<u8>,
}

#[derive(
	Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug
)]
pub enum StorageDeposit<Balance> {
	Refund(Balance), Charge(Balance)
}

impl DecodeWithMetadata for DispatchError {
    fn decode_with_metadata(bytes: &mut &[u8], metadata: &Metadata) -> Result<Self, subxt::Error> {
		let prev_count = bytes.len();
		let type_id = metadata.dispatch_error_ty().ok_or(MetadataError::DispatchErrorNotFound)?;
        let value = decode_as_type(
			bytes, type_id, metadata.types()
		).map_err(|err| scale_decode::Error::from(err))?;
		let mut out = Vec::with_capacity(prev_count - bytes.len());
		encode_as_type(&value, type_id, metadata.types(), &mut out)?;
		Self::decode_from(out, metadata.clone())
    }
}

#[derive(Debug)]
pub enum DispatchResult<T: Decode> {
	Ok(T), Err(DispatchError)
}

impl<T: Decode> From<DispatchResult<T>> for Result<T, DispatchError> {
    fn from(value: DispatchResult<T>) -> Self {
        match value {
			DispatchResult::Ok(val) => Ok(val),
			DispatchResult::Err(err) => Err(err)
		}
    }
}

impl<T: Decode> DecodeWithMetadata for DispatchResult<T> {
    fn decode_with_metadata(bytes: &mut &[u8], metadata: &Metadata) -> Result<Self, subxt::Error> {
        let byte = bytes.read_byte()?;
		match byte {
			0 => Ok(DispatchResult::Ok(T::decode(bytes)?)),
			1 => Ok(DispatchResult::Err(DispatchError::decode_with_metadata(bytes, metadata)?)),
			_ => Err(subxt::Error::Codec("Unknown Result enum index".into()))
		}
    }
}

pub trait DecodeWithMetadata: Sized {
	fn decode_with_metadata(bytes: &mut &[u8], metadata: &Metadata) -> Result<Self, subxt::Error>;
}