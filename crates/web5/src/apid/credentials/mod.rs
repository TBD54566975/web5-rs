pub mod presentation_definition;
pub mod verifiable_credential_11;

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
}

type Result<T> = std::result::Result<T, CredentialError>;
