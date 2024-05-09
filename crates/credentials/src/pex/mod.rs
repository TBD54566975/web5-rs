use crate::vc::CredentialError;

pub mod presentation_definition;

#[derive(thiserror::Error, Debug)]
pub enum PexError {
    #[error(transparent)]
    CredentialError(#[from] CredentialError),
    #[error("Failed to parse JSON: {0}")]
    JsonError(String),
}

type Result<T> = std::result::Result<T, PexError>;
