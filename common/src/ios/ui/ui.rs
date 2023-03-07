use crate::ui::UIProtocol;
use crate::error::Result as PCResult;

use super::sui::SUI;

pub (crate) struct UI {
    swift: SUI
}

impl UI {
    pub (crate) fn new(sui: SUI) -> Self {
        UI {
            swift: sui
        }
    }
}

impl UIProtocol for UI {
    fn present_error(&self, message: &str) -> PCResult<()> {
        self.swift.present_error(message)
    }
}