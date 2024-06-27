pub mod document;
pub mod service;
pub mod verification_method;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum DataModelError {
    #[error("publicKeyJwk not found {0}")]
    MissingPublicKeyJwk(String),
}

type Result<T> = std::result::Result<T, DataModelError>;
