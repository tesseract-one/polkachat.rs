use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub (crate) trait UIProtocol {
    fn present_error(&self, message: &str) -> Result<()>;
}

#[cfg(target_os = "android")]
pub (crate) use crate::android::UI;

#[cfg(target_os = "ios")]
pub (crate) use crate::ios::UI;