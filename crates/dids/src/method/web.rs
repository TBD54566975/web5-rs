use crate::did::Did;
use crate::method::{DidMethod, DidMethodError, DidResolutionResult};
use async_trait::async_trait;
use crypto::key_manager::KeyManager;
use did_web::DIDWeb as SpruceDidWebMethod;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use std::sync::Arc;

/// Concrete implementation for a did:web DID
pub struct DidWeb {
    uri: String,
    key_manager: Arc<dyn KeyManager>,
}

impl Did for DidWeb {
    fn uri(&self) -> &str {
        &self.uri
    }

    fn key_manager(&self) -> &Arc<dyn KeyManager> {
        &self.key_manager
    }
}

/// Options that can be used to create a did:web DID.
/// This is currently a unit struct because did:web does not support key creation.
pub struct DidWebCreateOptions;

#[async_trait]
impl DidMethod<DidWeb, DidWebCreateOptions> for DidWeb {
    const NAME: &'static str = "web";

    fn create(
        _key_manager: Arc<dyn KeyManager>,
        _options: DidWebCreateOptions,
    ) -> Result<DidWeb, DidMethodError> {
        Err(DidMethodError::DidCreationFailure(
            "create operation not supported for did:web".to_string(),
        ))
    }

    async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let input_metadata = ResolutionInputMetadata::default();
        let (did_resolution_metadata, did_document, did_document_metadata) =
            SpruceDidWebMethod.resolve(did_uri, &input_metadata).await;

        DidResolutionResult {
            did_resolution_metadata,
            did_document,
            did_document_metadata,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fails() {
        let key_manager = Arc::new(crypto::key_manager::LocalKeyManager::new_in_memory());
        let result = DidWeb::create(key_manager, DidWebCreateOptions);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolution_success() {
        let did_uri = "did:web:tbd.website";
        let result = DidWeb::resolve_uri(did_uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.expect("did_document not found");
        assert_eq!(did_document.id, did_uri);
    }

    #[tokio::test]
    async fn resolution_failure() {
        let result = DidWeb::resolve_uri("did:web:does-not-exist").await;
        assert!(result.did_resolution_metadata.error.is_some());
    }
}
