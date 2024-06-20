use super::{document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata};
use crate::apid::dids::{
    data_model::document::Document, did::Did, methods::did_jwk::DidJwk,
    resolution::resolution_metadata::ResolutionMetadataError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolutionResult {
    pub resolution_metadata: ResolutionMetadata,
    pub document: Option<Document>,
    pub document_metadata: Option<DocumentMetadata>,
}

impl ResolutionResult {
    pub fn new(uri: &str) -> Self {
        let did = Did::new(uri).unwrap(); // 🚧

        match did.method.as_str() {
            "jwk" => DidJwk::resolve(uri),
            _ => ResolutionResult {
                resolution_metadata: ResolutionMetadata {
                    error: Some(ResolutionMetadataError::MethodNotSupported),
                },
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_resolve_did_jwk() {
        let did_uri = "did:jwk:eyJrdHkiOiJPS1AiLCJ1c2UiOiJzaWciLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoiVnRTSFhQbEtEdzFFRW9PajVYTjNYV2hqU1BZVk52WC1lNHZqUk8weVlKQSIsIngiOiJpejcwc3ZTTHhOWmhzRHhlSlFfam5PVmJYM0tGTmtjQmNNaldqWm1YRXNBIiwiYWxnIjoiRWREU0EifQ";
        let resolution_result = ResolutionResult::new(did_uri);

        assert_eq!(None, resolution_result.resolution_metadata.error);
        assert_eq!(resolution_result.document.unwrap().id, did_uri.to_string());
    }
}
