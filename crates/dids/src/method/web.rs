use crate::{
    bearer::BearerDid,
    method::{Method, MethodError, ResolutionResult},
};
use async_trait::async_trait;
use crypto::key_manager::KeyManager;
use did_web::DIDWeb as SpruceDidWebMethod;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use std::sync::Arc;

/// Concrete implementation for a did:web DID
pub struct DidWeb {}

/// Options that can be used to create a did:web DID.
/// This is currently a unit struct because did:web does not support key creation.
pub struct DidWebCreateOptions;

#[async_trait]
impl Method<DidWebCreateOptions> for DidWeb {
    const NAME: &'static str = "web";

    fn create<T: KeyManager>(
        _key_manager: Arc<T>,
        _options: DidWebCreateOptions,
    ) -> Result<BearerDid<T>, MethodError> {
        Err(MethodError::DidCreationFailure(
            "create operation not supported for did:web".to_string(),
        ))
    }

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
    use crypto::key_manager::local_jwk_manager::LocalJwkManager;

    #[test]
    fn create_fails() {
        let key_manager = Arc::new(LocalJwkManager::new_in_memory());
        let result = DidWeb::create(key_manager, DidWebCreateOptions);
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
