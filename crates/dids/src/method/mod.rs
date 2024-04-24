pub mod jwk;
pub mod spruce_mappers;
pub mod web;

use crate::bearer::BearerDid;
use crate::resolver::ResolutionResult;
use keys::{
    key::KeyError,
    key_manager::{KeyManager, KeyManagerError},
};
use std::{future::Future, sync::Arc};

/// Errors that can occur when working with DID methods.
#[derive(thiserror::Error, Debug)]
pub enum MethodError {
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error("Failure creating DID: {0}")]
    DidCreationFailure(String),
}

/// A trait with common behavior across all DID methods.
pub trait Method<CreateOptions> {
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
    ) -> Result<BearerDid, MethodError>;

    /// Resolve a DID URI to a [`DidResolutionResult`], as specified in
    /// [Resolving a DID](https://w3c-ccg.github.io/did-resolution/#resolving).
    fn resolve(did_uri: &str) -> impl Future<Output = ResolutionResult>;
}
