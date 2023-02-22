use thiserror::Error;

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
    Tesseract(#[from] tesseract::Error)
}

pub (crate) type Result<T> = std::result::Result<T, Error>;