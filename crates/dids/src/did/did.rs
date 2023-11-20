use crate::resolver::{DidResolutionResult, DidResolver};
use async_trait::async_trait;
use crypto::key_manager::{KeyManager, KeyManagerError};
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum DidError {
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error("DID creation failed")]
    DidCreationFailed,
}

#[async_trait]
pub trait Did {
    fn uri(&self) -> &str;
    fn key_manager(&self) -> Arc<dyn KeyManager>;

    async fn resolve(&self) -> DidResolutionResult {
        DidResolver.resolve(&self.uri()).await
    }
}
