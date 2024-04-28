pub mod dht;
pub mod jwk;
pub mod spruce_mappers;
pub mod web;

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
    #[error("Failure publishing DID: {0}")]
    DidPublishingFailure(String),
}

/// Resolve is a trait for DID methods, so that a DID Document can be resolved from a DID URI.
pub trait Resolve {
    /// Resolve a DID URI to a [`DidResolutionResult`], as specified in
    /// [Resolving a DID](https://w3c-ccg.github.io/did-resolution/#resolving).
    fn resolve(did_uri: &str) -> impl Future<Output = ResolutionResult>;
}

/// Create is a trait for DID methods that can create DID methods. This is not enforced by the
/// Method trait, but is a supported DID method that many methods.
pub trait Create<O> {
    /// Create a new DID document and return the identifier.
    fn create(
        key_manager: Arc<dyn KeyManager>,
        opts: O,
    ) -> Result<crate::bearer::BearerDid, MethodError>;
}

/// Method is the trait for DID methods overall that can be resolved. Methods can also implement
/// the Create trait to allow for DID creation, but it is not enforced by the Method trait.
pub trait Method<T: Resolve = Self> {
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
}
