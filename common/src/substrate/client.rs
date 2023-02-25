use std::sync::Arc;

use async_trait::async_trait;

use http::Uri;
use jsonrpsee_core::client::{CertificateStore, ClientBuilder};
use jsonrpsee_client_transport::ws::WsTransportClientBuilder;
use subxt::{
    error::RpcError,
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
        let url: Uri = url
            .parse()
            .map_err(|e| RpcError::ClientError(Box::new(e)))?;
        debug!("\tUri created");
        let (sender, receiver) = WsTransportClientBuilder::default()
            .certificate_store(CertificateStore::WebPki)
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
            .max_notifs_per_subscription(4096)
            .build_with_tokio(sender, receiver);
        debug!("\tClient created");
        OnlineClient::from_rpc_client(Arc::new(client)).await
    }
}