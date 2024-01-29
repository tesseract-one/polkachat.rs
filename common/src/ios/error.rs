use tesseract_swift::error::TesseractSwiftError;
use tesseract_swift::utils::error::CError;

use crate::error::Error;

#[repr(u32)]
enum AppErrorCode {
    PublicKey,
    Substrate,
    TokioJoin,
    IO
}

impl From<Error> for TesseractSwiftError {
    fn from(value: Error) -> Self {
        match value {
            Error::IOS(err) => err,
            Error::LoggerInit(err) => Self::Logger(err.to_string()),
            Error::IO(err) => Self::Custom(AppErrorCode::IO as u32, err.to_string()),
            Error::PublicKey => Self::Custom(AppErrorCode::PublicKey as u32, Error::PublicKey.to_string()),
            Error::Substrate(err) => Self::Custom(AppErrorCode::Substrate as u32, err.to_string()),
            Error::TokioJoin(err) => Self::Custom(AppErrorCode::TokioJoin as u32, err.to_string())
        }
    }
}

impl From<Error> for CError {
    fn from(value: Error) -> Self {
        let tesseract: TesseractSwiftError = value.into();
        tesseract.into()
    }
}

impl From<CError> for Error {
    fn from(value: CError) -> Self {
        let tesseract: TesseractSwiftError = value.into();
        tesseract.into()
    }
}

impl From<tesseract_one::Error> for Error {
    fn from(value: tesseract_one::Error) -> Self {
        let tesseract: TesseractSwiftError = value.into();
        tesseract.into()
    }
}