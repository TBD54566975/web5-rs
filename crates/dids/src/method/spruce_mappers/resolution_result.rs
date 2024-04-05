use crate::document::DidDocument;
use crate::resolver::{
    DidDocumentMetadata, DidResolutionError, DidResolutionMetadata, DidResolutionResult,
};
use ssi_dids::did_resolve::{
    DocumentMetadata as SpruceDocumentMetadata, ResolutionMetadata as SpruceResolutionMetadata,
};
use ssi_dids::Document as SpruceDocument;

impl DidResolutionResult {
    pub fn from_spruce(
        spruce_resolution_metadata: SpruceResolutionMetadata,
        spruce_document: Option<SpruceDocument>,
        spruce_document_metadata: Option<SpruceDocumentMetadata>,
    ) -> Result<Self, DidResolutionError> {
        let did_resolution_metadata =
            match DidResolutionMetadata::from_spruce(spruce_resolution_metadata) {
                Ok(r) => r,
                Err(_) => return Err(DidResolutionError::InternalError),
            };

        let did_document = match spruce_document {
            Some(doc) => match DidDocument::from_spruce(doc) {
                Ok(d) => Some(d),
                Err(_) => return Err(DidResolutionError::InternalError),
            },
            None => None,
        };

        let did_document_metadata = match spruce_document_metadata {
            Some(doc) => match DidDocumentMetadata::from_spruce(doc) {
                Ok(d) => Some(d),
                Err(_) => return Err(DidResolutionError::InternalError),
            },
            None => None,
        };

        Ok(DidResolutionResult {
            did_resolution_metadata,
            did_document,
            did_document_metadata,
            ..Default::default()
        })
    }
}
