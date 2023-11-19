use dids::method::{DidCreationError, DidMethodError};
use dids::parsed_did::ParsedDidError;
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

impl From<DidMethodError> for DidsError {
    fn from(e: DidMethodError) -> Self {
        Self::new(e)
    }
}

impl From<DidResolutionError> for DidsError {
    fn from(e: DidResolutionError) -> Self {
        Self::new(e)
    }
}

impl From<ParsedDidError> for DidsError {
    fn from(e: ParsedDidError) -> Self {
        Self::new(e)
    }
}

impl From<DidCreationError> for DidsError {
    fn from(e: DidCreationError) -> Self {
        Self::new(e)
    }
}
