use super::{document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata};
use crate::apid::dids::data_model::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolutionResult {
    pub resolution_metadata: ResolutionMetadata,
    pub document: Option<Document>,
    pub document_metadata: Option<DocumentMetadata>,
}

impl ResolutionResult {
    pub fn new(uri: &str) -> Self {
        println!("ResolutionResult::new() called with {}", uri);
        Self {
            ..Default::default()
        }
    }
}
