mod create;
mod credential_schema;
mod credential_subject;
mod data_model_validation;
mod decode;
mod issuer;
mod josekit;
mod jwt_payload_vc;
pub mod presentation_definition;
mod sign;
mod status_list_credential;
pub mod verifiable_credential_1_1;

pub use credential_schema::CredentialSchema;
pub use credential_subject::CredentialSubject;
pub use issuer::{Issuer, ObjectIssuer};
pub use status_list_credential::{
    StatusListCredential, STATUS_LIST_2021, STATUS_LIST_2021_ENTRY, STATUS_LIST_CREDENTIAL_CONTEXT,
    STATUS_LIST_CREDENTIAL_TYPE,
};

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
