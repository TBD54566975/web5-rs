use crate::resolver::{DidResolutionResult, DidResolver};
use async_trait::async_trait;
use crypto::key_manager::KeyManager;
use std::sync::Arc;

/// Trait that defines all common behavior for a DID.
#[async_trait]
pub trait Did {
    fn uri(&self) -> &str;
    fn key_manager(&self) -> &Arc<dyn KeyManager>;

    async fn resolve(&self) -> DidResolutionResult {
        DidResolver::resolve_uri(self.uri()).await
    }
}
