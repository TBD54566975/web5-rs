pub mod jwk;

use crate::did::Did;
use crate::resolver::DidResolutionResult;
use async_trait::async_trait;
use crypto::key_manager::{KeyManager, KeyManagerError};
use std::sync::Arc;

/// Errors that can occur when working with DID methods.
#[derive(thiserror::Error, Debug)]
pub enum DidMethodError {
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error("Failure creating DID: {0}")]
    DidCreationFailure(String),
}

/// A trait with common behavior across all DID methods.
#[async_trait]
pub trait DidMethod<T: Did, CreateOptions> {
    const NAME: &'static str;

    /// Create a new DID instance.
    fn create(
        key_manager: Arc<dyn KeyManager>,
        options: CreateOptions,
    ) -> Result<T, DidMethodError>;

    /// Resolve a DID URI to a DID Document.
    async fn resolve_uri(did_uri: &str) -> DidResolutionResult;
}
