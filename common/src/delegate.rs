use std::collections::HashMap;

use async_trait::async_trait;

use tesseract::client::{Delegate, transport::Status};

use crate::{UI, UIProtocol};

pub (super) struct AppDelegate {
    ui: UI
}

impl AppDelegate {
    pub (super) fn new(ui: UI) -> Self {
        Self {
            ui
        }
    }
}

#[async_trait]
impl Delegate for AppDelegate {
    async fn select_transport(&self, transports: &HashMap<String, Status>) -> Option<String> {
        assert_eq!(1, transports.len(), "How the heck do we have more than one transport here?");
        let tid = transports.keys().next().map(String::clone).unwrap();

        let status = &transports[&tid];

        match status {
            Status::Ready => Some(tid),
            Status::Unavailable(reason) => {
                let _ = self.ui.present_error(&format!("Transport '{}' is not available because of the following reason: {}", tid, reason)).
                    inspect_err(|e| debug!("tried presenting an error to the user, but miseribly failed: {e}\nThe error was: {reason}"));
                None
            },
            Status::Error(e) => {
                let error = e.to_string();
                let _ = self.ui.present_error(&format!("Transport '{}' is not available because the transport produced an error: {}", tid, error)).
                    inspect_err(|e| debug!("tried presenting an error to the user, but miseribly failed: {e}\nThe error that happened to transport '{tid}' was: {error}"));
                None
            },
        }
    }
}