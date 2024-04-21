use crate::document::Document;
use crate::resolver::{DocumentMetadata, ResolutionError, ResolutionMetadata, ResolutionResult};
use ssi_dids::did_resolve::{
    DocumentMetadata as SpruceDocumentMetadata, ResolutionMetadata as SpruceResolutionMetadata,
};
use ssi_dids::Document as SpruceDocument;

impl ResolutionResult {
    pub fn from_spruce(
        spruce_resolution_metadata: SpruceResolutionMetadata,
        spruce_document: Option<SpruceDocument>,
        spruce_document_metadata: Option<SpruceDocumentMetadata>,
    ) -> Result<Self, ResolutionError> {
        let did_resolution_metadata =
            match ResolutionMetadata::from_spruce(spruce_resolution_metadata) {
                Ok(r) => r,
                Err(_) => return Err(ResolutionError::InternalError),
            };

        let did_document = match spruce_document {
            Some(doc) => match Document::from_spruce(doc) {
                Ok(d) => Some(d),
                Err(_) => return Err(ResolutionError::InternalError),
            },
            None => None,
        };

        let did_document_metadata = match spruce_document_metadata {
            Some(doc) => match DocumentMetadata::from_spruce(doc) {
                Ok(d) => Some(d),
                Err(_) => return Err(ResolutionError::InternalError),
            },
            None => None,
        };

        Ok(ResolutionResult {
            did_resolution_metadata,
            did_document,
            did_document_metadata,
            ..Default::default()
        })
    }
}
