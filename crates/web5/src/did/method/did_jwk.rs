use crate::crypto::key_manager::KeyManager;
use crate::did::parse::Did;
use crate::did::resolver::{DidResolutionError, DidResolutionResult, DidResolver};
use async_trait::async_trait;
use ssi_dids::did_resolve::DIDResolver;
use std::sync::Arc;

pub struct DidJwk {
    did: Did,
    key_manager: Arc<dyn KeyManager>,
}

#[async_trait]
impl DidResolver for DidJwk {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = did_jwk::DIDJWK
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok(DidResolutionResult {
            did_document,
            did_document_metadata,
            resolution_metadata,
        })
    }
}
