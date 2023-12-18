use std::{future::Future, sync::Arc};

use futures::lock::Mutex;
use tokio::runtime::Runtime;

use tesseract_one::client::{Service, Tesseract};
use tesseract_protocol_substrate::{AccountType, Substrate, SubstrateService};

use subxt::Metadata;

use crate::{
    contract::Api,
    delegate::AppDelegate,
    substrate::{Account, Credentials, TesseractSigner},
    Error, Result, UI,
};

pub(crate) struct Core {
    runtime: Runtime,
    tesseract: Arc<dyn Service<Protocol = Substrate>>,
    api: Mutex<Option<Arc<Api>>>,
    credentials: Mutex<Option<Credentials>>,
}

impl Core {
    pub(crate) fn new<F: FnOnce(Tesseract) -> Tesseract>(
        ui: UI,
        runtime: Runtime,
        apply_transports: F,
    ) -> Self {
        let delegate = AppDelegate::new(ui);
        let tesseract = Tesseract::new(Arc::new(delegate));

        let tesseract = apply_transports(tesseract);
        let service = tesseract.service(Substrate::Protocol);

        info!("Core created successfully");

        Self {
            runtime,
            tesseract: service,
            api: Mutex::new(None),
            credentials: Mutex::new(None),
        }
    }

    pub(crate) async fn account(self: Arc<Self>) -> Result<Account> {
        self.do_with_credentials(|credentials| {
            futures::future::ready(Ok(credentials.account.clone()))
        })
        .await
    }

    pub(crate) async fn account_string(self: Arc<Self>) -> Result<String> {
        use subxt_signer::sr25519::PublicKey;
        let raw_pk = self.account().await?.public_key;
        let pk = PublicKey(raw_pk.try_into().map_err(|_| Error::PublicKey)?);
        Ok(pk.to_account_id().to_string())
    }

    pub(crate) async fn messages(self: Arc<Self>, from: u32) -> Result<Vec<String>> {
        debug!("About to featch messages");
        let api = Arc::clone(&self).api().await?;
        debug!("Got the API to fetch messages");
        Ok(self
            .runtime
            .spawn(async move {
                debug!("Fetching messages inside runtime");
                let len = api.len().await?;
                api.get(from, len)
                    .await
                    .map(|msgs| msgs.into_iter().map(|msg| msg.text).collect())
            })
            .await??)
    }

    pub(crate) async fn send(
        self: Arc<Self>,
        message: String, /*need owned here*/
    ) -> Result<()> {
        let message = message.to_owned();
        debug!("About to send a new message");
        let api = Arc::clone(&self).api().await?;
        debug!("Got the API to send new message");
        let signer = Arc::clone(&self).signer().await?;
        debug!("Got the Signer to sign new message transaction");
        Ok(self
            .runtime
            .spawn(async move {
                debug!("Sending new message inside runtime");
                let signer: &TesseractSigner = &signer;
                api.add(&message, signer).await.map(|_| {})
            })
            .await??)
    }

    async fn signer(self: Arc<Self>) -> Result<Arc<TesseractSigner>> {
        self.do_with_credentials(|credentials| {
            futures::future::ready(Ok(Arc::clone(&credentials.signer)))
        })
        .await
    }

    async fn api(self: Arc<Self>) -> Result<Arc<Api>> {
        debug!("About to get API ref");
        let mut lock = self.api.lock().await;
        debug!("Aquired API lock");

        if lock.is_none() {
            debug!("API is not initialized yet. About to create a new instane");
            let api = self
                .runtime
                .spawn(async move {
                    debug!("Create a new API instane inside Runtime");
                    Api::new().await
                })
                .await??;

            debug!("A new API instane is successfully");
            *lock = Some(Arc::new(api));
            debug!("A new API instane is written to lock");
        }

        debug!("About to provide a new API ref");

        let api = lock
            .as_ref()
            .expect("All good. Can't be null. Can't use get_or_insert_with due to async");

        debug!("A new API ref is successfully provided");
        Ok(Arc::clone(&api))
    }

    async fn fetch_metadata(self: Arc<Self>) -> Result<Metadata> {
        Ok(self.api().await?.metadata())
    }

    async fn fetch_account(&self) -> Result<Account> {
        Ok(Arc::clone(&self.tesseract)
            .get_account(AccountType::Sr25519)
            .await?)
    }

    async fn fetch_credentials(self: Arc<Self>) -> Result<Credentials> {
        Credentials::new(
            self.fetch_account().await?,
            Arc::clone(&self.tesseract),
            self.fetch_metadata().await?,
        )
    }

    async fn do_with_credentials<T, F: Future<Output = Result<T>>>(
        self: Arc<Self>,
        f: impl FnOnce(&Credentials) -> F,
    ) -> Result<T> {
        let this = Arc::clone(&self); //need this because of lock
        let mut lock = this.credentials.lock().await;

        if lock.is_none() {
            *lock = Some(self.fetch_credentials().await?);
        }

        let credentials = lock
            .as_ref()
            .expect("All good. Can't be null. Can't use get_or_insert_with due to async");
        f(credentials).await
    }
}
