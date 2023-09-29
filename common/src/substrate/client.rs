use async_trait::async_trait;

use jsonrpsee_core::client::ClientBuilder;
use jsonrpsee_client_transport::ws::{Url, WsTransportClientBuilder};
use subxt::{
    error::RpcError, backend::rpc::RpcClient,
    Config, Error, OnlineClient
};

#[async_trait]
pub (crate) trait OnlineClientWebPKI {
    type Conf: Config;

    async fn from_url_web_pki(url: &str) -> Result<OnlineClient<Self::Conf>, Error>;
}

#[async_trait]
impl<T: Config> OnlineClientWebPKI for OnlineClient<T> {
    type Conf = T;

    async fn from_url_web_pki(url: &str) -> Result<OnlineClient<Self::Conf>, Error> {
        debug!("Creating WebPKI Susbtrate OnlineClient");
        let url: Url = url
            .parse()
            .map_err(|e| RpcError::ClientError(Box::new(e)))?;
        debug!("\tUri created");
        let (sender, receiver) = WsTransportClientBuilder::default()
            .use_webpki_rustls()
            .build(url)
            .await
            .inspect_err(|e| {
                debug!("WebPKI Error: {:?}", &e);
            })
            .map_err(|e| {
                RpcError::ClientError(Box::new(e))
            })?;
        debug!("\tBuilder created");
        let client = ClientBuilder::default()
            .max_buffer_capacity_per_subscription(4096)
            .build_with_tokio(sender, receiver);
        debug!("\tClient created");
        OnlineClient::from_rpc_client(RpcClient::new(client)).await
    }
}