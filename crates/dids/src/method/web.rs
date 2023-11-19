use crate::resolver::{DidResolutionError, DidResolutionResponse, DidResolver};
use crate::Did;
use async_trait::async_trait;
use did_web::DIDWeb;
use ssi_dids::did_resolve::DIDResolver;

pub struct DidWebData {}

pub type DidWeb = Did<DidWebData>;

#[async_trait]
impl DidResolver for DidWeb {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResponse, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = DIDWeb
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok((resolution_metadata, did_document, did_document_metadata))
    }
}
