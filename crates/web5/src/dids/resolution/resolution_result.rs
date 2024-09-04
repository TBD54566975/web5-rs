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
    pub async fn resolve(uri: &str) -> Self {
        let did = match Did::parse(uri) {
            Ok(did) => did,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        match did.method.as_str() {
            "jwk" => DidJwk::resolve(uri),
            "dht" => DidDht::resolve(uri, None).await,
            "web" => DidWeb::resolve(uri).await,
            _ => ResolutionResult::from(ResolutionMetadataError::MethodNotSupported),
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
    use mockito::Server;

    mod resolve {
        use super::*;

        #[tokio::test]
        async fn test_invalid_did() {
            let resolution_result = ResolutionResult::resolve("something invalid").await;
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[tokio::test]
        async fn test_did_jwk() {
            let bearer_did = DidJwk::create(None).unwrap();

            let resolution_result = ResolutionResult::resolve(&bearer_did.did.uri).await;
            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert_eq!(resolution_result.document.unwrap(), bearer_did.document);
        }

        #[tokio::test]
        async fn test_did_web() {
            let mut mock_server = Server::new_async().await;
            let url = mock_server.url();

            let bearer_did = DidWeb::create(&url, None).unwrap();

            let _ = mock_server
                .mock("GET", "/.well-known/did.json")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(&bearer_did.document).unwrap())
                .create_async()
                .await;

            let resolution_result = ResolutionResult::resolve(&bearer_did.did.uri).await;

            assert_eq!(resolution_result.resolution_metadata.error, None);
            assert!(resolution_result.document.is_some());
            assert_eq!(resolution_result.document.unwrap(), bearer_did.document);
        }

        #[tokio::test]
        async fn test_method_not_supported() {
            let resolution_result = ResolutionResult::resolve("did:example:123").await;
            assert!(resolution_result.resolution_metadata.error.is_some());
            assert_eq!(
                resolution_result.resolution_metadata.error.unwrap(),
                ResolutionMetadataError::MethodNotSupported
            );
        }
    }
}
