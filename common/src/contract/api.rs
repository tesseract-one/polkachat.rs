use std::str::FromStr;
use std::sync::Arc;

use subxt::{
    ext::codec::Decode, utils::AccountId32, tx::Signer,
    Config, Error, OnlineClient, Metadata, PolkadotConfig
};

use crate::substrate::OnlineClientWebPKI;
use super::primitives::Weight;

use super::call::*;

mod contract {
    use super::Decode;
    use super::Weight;

    #[derive(Decode, Debug)]
    pub struct Message<AccountId: Decode> {
        pub id: u32,
        pub sender: AccountId,
        pub text: String,
    }

    #[derive(Decode, Debug)]
    pub enum Events<AccountId: Decode> {
        MessageAdded(Message<AccountId>),
    }

    pub mod calls {
        pub const ADD: [u8; 4] = [0x4b, 0x05, 0x0e, 0xa9];
        pub const GET: [u8; 4] = [0x2f, 0x86, 0x5b, 0xd9];
        pub const LEN: [u8; 4] = [0x83, 0x9b, 0x35, 0x48];
    }

    // Can be imported form ink_primitives but added here to reduce dependencies
    #[non_exhaustive]
    #[repr(u32)]
    #[derive(Decode, Debug)]
    pub enum LangError {
        CouldNotReadInput,
    }

    pub const GAS_LIMIT: Weight = Weight::from_parts(9_375_000_000, 524288);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Message {
    pub id: u32,
    pub sender: AccountId32,
    pub text: String,
}

impl From<contract::Message<AccountId32>> for Message {
    fn from(message: contract::Message<AccountId32>) -> Self {
        Self {
            id: message.id,
            sender: message.sender,
            text: message.text,
        }
    }
}

pub(crate) struct Api {
    client: OnlineClient<PolkadotConfig>,
    contract: AccountId32,
}

//if you need to publish on mainnet, please, just wrap with features
const API_URL: &str = "wss://rococo-contracts-rpc.polkadot.io:443";
const SMART_CONTRACT: &str = "5GZRb5XZVCTsH6VSxT3e8tE3qQmaiq4hJhxgdoFg8iijP3S9";

impl Api {
    pub(crate) async fn new() -> Result<Self, Error> {
        debug!("Creating Substrate client - start");
        let client = OnlineClient::<PolkadotConfig>::from_url_web_pki(API_URL).await?;
        debug!("Created client successfully");
        let contract = AccountId32::from_str(SMART_CONTRACT)
            .map_err(|err| Error::Other(err.to_string()))?;
        debug!("Created contract ID");
        Ok(Self { client, contract })
    }

    pub(crate) fn metadata(&self) -> Metadata {
        self.client.metadata()
    }

    pub(crate) async fn get(self: Arc<Self>, from: u32, to: u32) -> Result<Vec<Message>, Error> {
        let query = ContractCallQuery::<<PolkadotConfig as Config>::AccountId>::new_call(
            self.contract.clone(),
            self.contract.clone(),
            0,
            None,
            None,
            contract::calls::GET,
        )
        .add_parameter(from)
        .add_parameter(to);

        let hash = self.client.backend().latest_finalized_block_ref().await?;
        let response = self
            .client
            .backend()
            .call("ContractsApi_call", Some(&query.as_param()), hash.hash())
            .await?;

        let result: Result<
            Vec<contract::Message<<PolkadotConfig as Config>::AccountId>>,
            contract::LangError,
        > = parse_query_result(response, &self.client.metadata())?.0;
        let messages = result.map_err(|e| format!("{:?}", e))?;
        debug!("Messages {:?}", messages);
        Ok(messages.into_iter().map(Message::from).collect())
    }

    pub async fn len(&self) -> Result<u32, Error> {
        let query = ContractCallQuery::<<PolkadotConfig as Config>::AccountId>::new_call(
            self.contract.clone(),
            self.contract.clone(),
            0,
            None,
            None,
            contract::calls::LEN,
        );

        let hash = self.client.backend().latest_finalized_block_ref().await?;

        let response = self
            .client
            .backend()
            .call("ContractsApi_call", Some(&query.as_param()), hash.hash())
            .await?;
        
        let value: Result<u32, contract::LangError> 
            = parse_query_result(response, &self.client.metadata())?.0;
        Ok(value.map_err(|e| format!("{:?}", e))?)
    }

    pub async fn add<S>(&self, text: &str, signer: &S) -> Result<Message, Error>
    where
        S: Signer<PolkadotConfig> + Send + Sync,
    {
        let mut call = ContractCallCall::<<PolkadotConfig as Config>::Address>::new_call(
            self.contract.clone().into(),
            0,
            contract::GAS_LIMIT,
            None,
            contract::calls::ADD,
        );
        call = call.add_parameter(text);
        let tx = call.tx();
        let contract::Events::MessageAdded(msg) = self
            .client
            .tx()
            .sign_and_submit_then_watch_default(&tx, signer)
            .await?
            .wait_for_finalized_success()
            .await?
            .find_first::<ContractEmittedEvent<<PolkadotConfig as Config>::AccountId>>()?
            .ok_or("ContractEmitted event not found")?
            .try_parse_event::<contract::Events<<PolkadotConfig as Config>::AccountId>>()?;
        Ok(msg.into())
    }
}
