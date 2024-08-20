use crate::errors::Web5Error;

use super::resolution::resolution_metadata::ResolutionMetadataError;
use base64::DecodeError;
use serde_json::Error as SerdeJsonError;

pub mod did_dht;
pub mod did_web;

pub mod did_jwk;

#[derive(thiserror::Error, Debug)]
pub enum MethodError {
    #[error(transparent)]
    Web5Error(#[from] Web5Error),
    #[error("Failure creating DID: {0}")]
    DidCreationFailure(String),
    #[error("Failure publishing DID: {0}")]
    DidPublishingFailure(String),
    #[error("serde json error {0}")]
    SerdeJsonError(String),
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionMetadataError),
}

impl From<SerdeJsonError> for MethodError {
    fn from(err: SerdeJsonError) -> Self {
        MethodError::SerdeJsonError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, MethodError>;
