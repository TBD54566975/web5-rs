use crate::did::resolver::{DidResolutionError, DidResolutionResult, DidResolver};
use async_trait::async_trait;
use did_method_key::DIDKey;
use ssi_dids::did_resolve::DIDResolver;

pub struct DidKey {}

#[async_trait]
impl DidResolver for DidKey {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = DIDKey
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok(DidResolutionResult {
            resolution_metadata,
            did_document,
            did_document_metadata,
        })
    }
}
