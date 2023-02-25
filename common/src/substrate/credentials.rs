use std::sync::Arc;

use subxt::Metadata;
use tesseract::client::Service;
use tesseract_protocol_substrate::Substrate;

use crate::Result;

use super::account::Account;
use super::signer::TesseractSigner;

pub (crate) struct Credentials {
    pub account: Account,
    pub signer: Arc<TesseractSigner>
}

impl Credentials {
    pub (crate) fn new(account: Account, tesseract: Arc<dyn Service<Protocol = Substrate>>, metadata: Metadata) -> Result<Self> {
        let signer = TesseractSigner::new(tesseract, &account, metadata)?;
        Ok(Self {
            account: account,
            signer: Arc::new(signer)
        })
    }
}

