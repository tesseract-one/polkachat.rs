use thiserror::Error;

use tokio::task::JoinError;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[cfg(target_os = "android")]
    #[error("Android error")]
    Android(#[from] tesseract_android::error::TesseractAndroidError),

    #[cfg(target_os = "ios")]
    #[error("Logger initialization error")]
    LoggerInit(#[from] log::SetLoggerError),

    #[cfg(target_os = "ios")]
    #[error("C error")]
    CError(#[from] tesseract_utils::error::CError),

    #[error("Invalid public key")]
    PublicKey,

    #[error("Substrate error: {0}")]
    Substrate(#[from] subxt::Error),

    #[error("Tokio Join Error: {0}")]
    TokioJoin(#[from] JoinError),

    #[error("I/O Error: {0}")]
    IO(#[from] std::io::Error)
}

pub (crate) type Result<T> = std::result::Result<T, Error>;

impl Into<tesseract::Error> for Error {
    fn into(self) -> tesseract::Error {
        match self {
            #[cfg(target_os = "android")]
            Error::Android(e) => e.into(),
            Error::IO(e) => {
                let description = format!("IOError: {}", e);
                tesseract::Error::described(tesseract::ErrorKind::Weird, &description)
            },
            e => {
                let description = format!("Wallet error: {}", e);
                tesseract::Error::described(tesseract::ErrorKind::Weird, &description)
            }
        }
    }
}