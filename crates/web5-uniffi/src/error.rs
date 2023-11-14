use crate::key_manager::KeyStoreError;

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum Web5Error {
    #[error(transparent)]
    KeyStoreError(#[from] KeyStoreError),
    #[error(transparent)]
    JwkError(#[from] ssi_jwk::Error),
    #[error("Error resolving DID: {0}")]
    DidResolveError(String),
    #[error("An unknown error occurred")]
    Unknown,
}
