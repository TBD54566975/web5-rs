pub mod jwk;
pub mod key;
pub mod web;

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
    /// The name of the implemented DID method (e.g. `jwk`).
    ///
    /// This is used to identify the [`DidMethod`] responsible for creating/resolving an arbitrary
    /// DID URI.
    ///
    /// # Example
    /// If a consumer wants to resolve a DID URI of `did:jwk:12345`, the method portion of the URI
    /// (`jwk` in this example) is compared against each [`DidMethod`]'s `NAME` constant. If a match
    /// is found, the corresponding [`DidMethod`] is used to resolve the DID URI.
    const NAME: &'static str;

    /// Create a new DID instance.
    fn create(
        key_manager: Arc<dyn KeyManager>,
        options: CreateOptions,
    ) -> Result<T, DidMethodError>;

    /// Resolve a DID URI to a [`DidResolutionResult`], as specified in
    /// [Resolving a DID](https://w3c-ccg.github.io/did-resolution/#resolving).
    async fn resolve_uri(did_uri: &str) -> DidResolutionResult;
}
