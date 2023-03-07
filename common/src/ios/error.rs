use tesseract_utils::error::CError::ErrorCode;

use crate::error::Error;

#[repr(u32)]
enum DAErrorCodes {
    LoggerInit = 0x1000,
    Tesseract,
    PublicKey,
    Substrate,
    TokioJoin,
    IO
}

impl Into<tesseract_utils::error::CError> for crate::error::Error  {
    fn into(self) -> tesseract_utils::error::CError {
        match self {
            Error::CError(cerror) => cerror,
            Error::LoggerInit(logger_error) => ErrorCode(DAErrorCodes::LoggerInit as u32, logger_error.to_string().into()),
            Error::Tesseract(error) => ErrorCode(DAErrorCodes::Tesseract as u32, error.to_string().into()),
            Error::PublicKey => ErrorCode(DAErrorCodes::PublicKey as u32, Error::PublicKey.to_string().into()),
            Error::Substrate(error) => ErrorCode(DAErrorCodes::Substrate as u32, error.to_string().into()),
            Error::TokioJoin(error) => ErrorCode(DAErrorCodes::TokioJoin as u32, error.to_string().into()),
            Error::IO(error) => ErrorCode(DAErrorCodes::IO as u32, error.to_string().into()),
        }
    }
}