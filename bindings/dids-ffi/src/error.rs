use dids::did::DidError;
use dids::resolver::DidResolutionError;
use std::fmt::Display;
use uniffi::UnexpectedUniFFICallbackError;

pub type Result<T, E = DidsError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum DidsError {
    #[error("{message}")]
    InternalError { message: String },
}

impl DidsError {
    fn new<E: Display>(error: E) -> Self {
        Self::InternalError {
            message: error.to_string(),
        }
    }
}
impl From<UnexpectedUniFFICallbackError> for DidsError {
    fn from(e: UnexpectedUniFFICallbackError) -> Self {
        Self::new(e)
    }
}

impl From<DidResolutionError> for DidsError {
    fn from(e: DidResolutionError) -> Self {
        Self::new(e)
    }
}

impl From<DidError> for DidsError {
    fn from(e: DidError) -> Self {
        Self::new(e)
    }
}
