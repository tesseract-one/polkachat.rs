use std::str::FromStr;
use std::sync::Arc;

use sp_weights::Weight;
use subxt::{
    ext::codec::Decode, ext::sp_core::Bytes, ext::sp_runtime::AccountId32, rpc_params, tx::Signer,
    Config, OnlineClient, PolkadotConfig, Metadata, Error
};

use crate::substrate::OnlineClientWebPKI;

use super::call::*;

mod contract {
    use super::Decode;
    use super::Weight;
    use subxt::events::StaticEvent;

    #[derive(Decode)]
    pub struct AddEvent {}

    impl StaticEvent for AddEvent {
        const PALLET: &'static str = "Contracts";
        const EVENT: &'static str = "Called";
    }

    pub mod calls {
        pub const ADD: &'static str = "0x4b050ea9";
        pub const GET: &'static str = "0x2f865bd9";
        pub const LEN: &'static str = "0x839b3548";
    }

    pub const GAS_LIMIT: Weight = Weight::from_parts(9_375_000_000, 524288);
}

pub (crate) struct Api {
    client: OnlineClient<PolkadotConfig>,
    contract: AccountId32,
}

//if you need to publish on mainnet, please, just wrap with features
const API_URL: &str = "wss://rococo-contracts-rpc.polkadot.io:443";
const SMART_CONTRACT: &str = "5E5UJJ91pVa82RXnteAQV8ERMxZy5wW6fS2MpmRF3GXNpdjE";

impl Api {
    pub (crate) async fn new() -> Result<Self, Error> {
        debug!("Creating Substrate client - start");
        let client = OnlineClient::<PolkadotConfig>::from_url_web_pki(API_URL).await?;
        debug!("Created client successfully");
        let contract = AccountId32::from_str(SMART_CONTRACT)?;
        debug!("Created contract ID");
        Ok(Self {
            client,
            contract,
        })
    }

    pub (crate) fn metadata(&self) -> Metadata {
        self.client.metadata()
    }

    pub (crate) async fn get(self: Arc<Self>, from: u32, to: u32) -> Result<Vec<String>, Error> {
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

        let at: Option<<PolkadotConfig as Config>::Hash> = None;
        let params = rpc_params!["ContractsApi_call", query.as_param(), at];

        let response = self.client.rpc().request::<Bytes>("state_call", params).await?;

        let value: Vec<String> = parse_query_result(response)?.0;
        Ok(value)
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
        let at: Option<<PolkadotConfig as Config>::Hash> = None;
        let params = rpc_params!["ContractsApi_call", query.as_param(), at];

        let response = self.client.rpc().request::<Bytes>("state_call", params).await?;

        let value = parse_query_result(response)?.0;
        Ok(value)
    }

    pub async fn add<S>(&self, text: &str, signer: &S) -> Result<(), Error>
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
        self.client
            .tx()
            .sign_and_submit_then_watch_default(&tx, signer)
            .await?
            .wait_for_finalized_success()
            .await?
            .find_first::<contract::AddEvent>()?
            .ok_or("No event")?;
        Ok(())
    }
}