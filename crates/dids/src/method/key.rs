use crate::did::Did;
use crate::method::{DidMethod, DidMethodError, DidResolutionResult};
use async_trait::async_trait;
use crypto::key::{Key, KeyType};
use crypto::key_manager::KeyManager;
use did_method_key::DIDKey as SpruceDidKeyMethod;
use ssi_dids::did_resolve::{DIDResolver, ResolutionInputMetadata};
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

/// Concrete implementation for a did:key DID
pub struct DidKey {
    uri: String,
    key_manager: Arc<dyn KeyManager>,
}

pub struct DidKeyCreateOptions {
    pub key_type: KeyType,
}

impl Did for DidKey {
    fn uri(&self) -> &str {
        &self.uri
    }

    fn key_manager(&self) -> &Arc<dyn KeyManager> {
        &self.key_manager
    }
}

#[async_trait]
impl DidMethod<DidKey, DidKeyCreateOptions> for DidKey {
    const NAME: &'static str = "key";

    fn create(
        key_manager: Arc<dyn KeyManager>,
        options: DidKeyCreateOptions,
    ) -> Result<DidKey, DidMethodError> {
        let key_alias = key_manager.generate_private_key(options.key_type)?;
        let public_key =
            key_manager
                .get_public_key(&key_alias)?
                .ok_or(DidMethodError::DidCreationFailure(
                    "PublicKey not found".to_string(),
                ))?;

        let uri = SpruceDidKeyMethod
            .generate(&Source::Key(public_key.jwk()))
            .ok_or(DidMethodError::DidCreationFailure(
                "Failed to generate did:key".to_string(),
            ))?;

        Ok(DidKey { uri, key_manager })
    }

    async fn resolve_uri(did_uri: &str) -> DidResolutionResult {
        let input_metadata = ResolutionInputMetadata::default();
        let (did_resolution_metadata, did_document, did_document_metadata) =
            SpruceDidKeyMethod.resolve(did_uri, &input_metadata).await;

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
    use crypto::key_manager::local_key_manager::LocalKeyManager;
    use ssi_dids::did_resolve::ERROR_INVALID_DID;

    fn create_did_key() -> DidKey {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidKeyCreateOptions {
            key_type: KeyType::Ed25519,
        };

        DidKey::create(key_manager, options).expect("DidKey creation failed")
    }

    #[test]
    fn create_produces_correct_uri() {
        let did = create_did_key();
        assert!(did.uri.starts_with("did:key:"));
    }

    #[tokio::test]
    async fn instance_resolve() {
        let did = create_did_key();
        let result = did.resolve().await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.unwrap();
        assert_eq!(did_document.id, did.uri);
    }

    #[tokio::test]
    async fn resolve_uri_success() {
        let did = create_did_key();
        let result = DidKey::resolve_uri(&did.uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.expect("did_document not found");
        assert_eq!(did_document.id, did.uri);
    }

    #[tokio::test]
    async fn resolve_uri_failure() {
        let result = DidKey::resolve_uri("did:key:does-not-exist").await;
        assert_eq!(
            result.did_resolution_metadata.error,
            Some(ERROR_INVALID_DID.to_string())
        );
    }
}
