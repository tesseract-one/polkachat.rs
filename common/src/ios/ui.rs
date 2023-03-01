use async_trait::async_trait;

use crate::ui::UIProtocol;
use crate::error::Result as PCResult;

pub (crate) struct UI {
}

#[async_trait]
impl UIProtocol for UI {
    fn present_error(&self, message: &str) -> PCResult<()> {
        todo!()
    }
}