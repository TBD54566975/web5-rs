use crate::{
    bearer::BearerDid,
    method::{Method, MethodError, ResolutionResult},
};
use did_web::DIDWeb as SpruceDidWebMethod;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};

/// Concrete implementation for a did:web DID
pub struct DidWeb {}

impl DidWeb {
    pub fn create() -> Result<BearerDid, MethodError> {
        Err(MethodError::DidCreationFailure(
            "create operation not supported for did:web".to_string(),
        ))
    }
}

impl Method for DidWeb {
    const NAME: &'static str = "web";

    async fn resolve(did_uri: &str) -> ResolutionResult {
        let input_metadata = ResolutionInputMetadata::default();
        let (spruce_resolution_metadata, spruce_document, spruce_document_metadata) =
            SpruceDidWebMethod.resolve(did_uri, &input_metadata).await;

        match ResolutionResult::from_spruce(
            spruce_resolution_metadata,
            spruce_document,
            spruce_document_metadata,
        ) {
            Ok(r) => r,
            Err(e) => ResolutionResult::from_error(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fails() {
        let result = DidWeb::create();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolution_success() {
        let did_uri = "did:web:tbd.website";
        let result = DidWeb::resolve(did_uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.expect("did_document not found");
        assert_eq!(did_document.id, did_uri);
    }

    #[tokio::test]
    async fn resolution_failure() {
        let result = DidWeb::resolve("did:web:does-not-exist").await;
        assert!(result.did_resolution_metadata.error.is_some());
    }
}
