pub mod presentation_definition;
pub mod verifiable_credential_1_1;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum CredentialError {
    #[error("missing claim: {0}")]
    MissingClaim(String),
    #[error("claim mismatch: {0}")]
    ClaimMismatch(String),
    #[error("misconfigured expiration date: {0}")]
    MisconfiguredExpirationDate(String),
    #[error("credential expired")]
    CredentialExpired,
    #[error("data model validation error: {0}")]
    DataModelValidationError(String),
    #[error("missing kid jose header")]
    MissingKid,
}
