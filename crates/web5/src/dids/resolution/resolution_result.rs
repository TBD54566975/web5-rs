use super::{document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata};
use crate::dids::{
    data_model::document::Document,
    did::Did,
    methods::{did_dht::DidDht, did_jwk::DidJwk, did_web::DidWeb},
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
        let did = match Did::parse(uri) {
            Ok(did) => did,
            Err(_) => {
                return ResolutionResult {
                    resolution_metadata: ResolutionMetadata {
                        error: Some(ResolutionMetadataError::InvalidDid),
                    },
                    ..Default::default()
                }
            }
        };

        match did.method.as_str() {
            "jwk" => DidJwk::resolve(uri),
            "dht" => DidDht::resolve(uri),
            "web" => DidWeb::resolve(uri),
            _ => ResolutionResult {
                resolution_metadata: ResolutionMetadata {
                    error: Some(ResolutionMetadataError::MethodNotSupported),
                },
                ..Default::default()
            },
        }
    }
}

impl From<ResolutionMetadataError> for ResolutionResult {
    fn from(error: ResolutionMetadataError) -> Self {
        Self {
            resolution_metadata: ResolutionMetadata { error: Some(error) },
            ..Default::default()
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

    #[test]
    fn can_resolve_invalid_did_web() {
        let did_uri = "did:web:tbd.website";
        let resolution_result = ResolutionResult::new(did_uri);

        // the did:web we host is currently invalid hehe https://www.tbd.website/.well-known/did.json
        assert_eq!(
            Some(ResolutionMetadataError::RepresentationNotSupported),
            resolution_result.resolution_metadata.error
        );
    }

    // This works if you host a did web document json at http://localhost:1234/.well-known/did.json
    // #[test]
    // fn can_resolve_valid_did_web() {
    //     let did_uri = "did:web:localhost%3A1234";
    //     let resolution_result = ResolutionResult::new(did_uri);
    //
    //     assert_eq!(None, resolution_result.resolution_metadata.error);
    //     assert_eq!(resolution_result.document.unwrap().id, "did:web:tbd.website");
    // }

    #[test]
    fn can_resolve_did_dht() {
        let did_uri = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy";
        let resolution_result = ResolutionResult::new(did_uri);

        assert_eq!(None, resolution_result.resolution_metadata.error);
        assert_eq!(resolution_result.document.unwrap().id, did_uri.to_string());
    }
}
