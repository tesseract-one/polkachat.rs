use std::sync::Arc;

use futures::lock::Mutex;
use tesseract::client::{Service, Tesseract, Delegate};
use tesseract_protocol_substrate::{Substrate, SubstrateService, AccountType};

use crate::delegate::AppDelegate;

use crate::error::Result;

pub (crate) type Account = tesseract_protocol_substrate::GetAccountResponse;

pub (crate) struct Core {
    tesseract: Arc<dyn Service<Protocol = Substrate>>,
    account: Mutex<Option<Account>>
}

impl Core {
    pub (crate) fn new<F: FnOnce(Tesseract<AppDelegate>)->Tesseract<AppDelegate>>(/*ui:UI, */apply_transports: F) -> Self {
        //let ui = Arc::new(ui);

        let delegate = AppDelegate::new();
        let tesseract = Tesseract::new(Arc::new(delegate));

        let tesseract = apply_transports(tesseract);
        let service = tesseract.service(Substrate::Protocol);

        info!("Core created successfully");

        Self {
            tesseract: service,
            account: Mutex::new(None)
        }
    }

    pub (crate) async fn account(self: Arc<Self>) -> Result<Account> {
        let mut lock = self.account.lock().await;

        let account = lock.clone();

        if let Some(account) = account {
            return Ok(account);
        }

        let account = Arc::clone(&self.tesseract).get_account(AccountType::Sr25519).await?;
        *lock = Some(account.clone());

        Ok(account)
    }
}