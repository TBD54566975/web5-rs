use std::time::SystemTimeError;

use josekit::JoseError as JosekitError;
use serde_json::Error as SerdeJsonError;

use super::dids::{
    bearer_did::BearerDidError, data_model::DataModelError, did::DidError,
    resolution::resolution_metadata::ResolutionMetadataError,
};

pub mod presentation_definition;
pub mod verifiable_credential_1_1;

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    #[error("missing claim: {0}")]
    MissingClaim(String),
    #[error("claim mismatch: {0}")]
    ClaimMismatch(String),
    #[error("misconfigured expiration date: {0}")]
    MisconfiguredExpirationDate(String),
    #[error("Credential expired")]
    CredentialExpired,
    #[error("VC data model validation error: {0}")]
    VcDataModelValidationError(String),
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
    #[error("serde json error {0}")]
    SerdeJsonError(String),
    #[error(transparent)]
    Jose(#[from] JosekitError),
    #[error(transparent)]
    BearerDid(#[from] BearerDidError),
    #[error("missing kid jose header")]
    MissingKid,
    #[error(transparent)]
    Resolution(#[from] ResolutionMetadataError),
    #[error(transparent)]
    DidDataModel(#[from] DataModelError),
    #[error(transparent)]
    Did(#[from] DidError),
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),
}

impl From<SerdeJsonError> for CredentialError {
    fn from(err: SerdeJsonError) -> Self {
        CredentialError::SerdeJsonError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, CredentialError>;
