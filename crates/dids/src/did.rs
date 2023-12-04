use crate::resolver::{DidResolutionResult, DidResolver};
use async_trait::async_trait;
use crypto::key_manager::KeyManager;
use std::sync::Arc;

/// Trait that defines all common behavior for a DID.
#[async_trait]
pub trait Did {
    /// Returns the DID URI the target [`Did`] represents (e.g: `did:jwk:12345`).
    fn uri(&self) -> &str;

    /// Returns a reference to the [`KeyManager`] that contains all the keys necessary to
    /// manage and sign using the target [`Did`].
    fn key_manager(&self) -> &Arc<dyn KeyManager>;

    /// Returns a [`DidResolutionResult`] for the target [`Did`], as specified in
    /// [Resolving a DID](https://w3c-ccg.github.io/did-resolution/#resolving).
    async fn resolve(&self) -> DidResolutionResult {
        DidResolver::resolve_uri(self.uri()).await
    }
}
