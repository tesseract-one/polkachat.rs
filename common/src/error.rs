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
    #[error("IOS error: {0}")]
    IOS(#[from] tesseract_swift::error::TesseractSwiftError),

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

impl Into<tesseract_one::Error> for Error {
    fn into(self) -> tesseract_one::Error {
        match self {
            #[cfg(target_os = "android")]
            Error::Android(e) => e.into(),
            #[cfg(target_os = "ios")]
            Error::IOS(e) => e.into(),
            Error::IO(e) => {
                let description = format!("IOError: {}", e);
                tesseract_one::Error::described(tesseract_one::ErrorKind::Weird, &description)
            },
            e => {
                let description = format!("Wallet error: {}", e);
                tesseract_one::Error::described(tesseract_one::ErrorKind::Weird, &description)
            }
        }
    }
}