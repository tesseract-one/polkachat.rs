use std::collections::HashMap;

use async_trait::async_trait;

use tesseract::client::{Delegate, transport::Status};

pub (super) struct AppDelegate {
}

impl AppDelegate {
    pub (super) fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Delegate for AppDelegate {
    async fn select_transport(&self, transports: &HashMap<String, Status>) -> Option<String> {
        transports.keys().next().map(String::clone)
    }
}