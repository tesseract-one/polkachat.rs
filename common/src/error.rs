use thiserror::Error;

use tokio::task::JoinError;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[cfg(target_os = "android")]
    #[error("JNI error")]
    JNI(#[from] jni::errors::Error),

    #[cfg(target_os = "ios")]
    #[error("Logger initialization error")]
    LoggerInit(#[from] log::SetLoggerError),

    #[cfg(target_os = "ios")]
    #[error("C error")]
    CError(#[from] tesseract_utils::error::CError),

    #[error("Tesseract error: {0}")]
    Tesseract(#[from] tesseract::Error),

    #[error("Invalid public key")]
    PublicKey,

    #[error("Substrate error: {0}")]
    Substrate(#[from] subxt::Error),

    #[error("Tokio Join Error: {0}")]
    TokioJoin(#[from] JoinError)
}

pub (crate) type Result<T> = std::result::Result<T, Error>;