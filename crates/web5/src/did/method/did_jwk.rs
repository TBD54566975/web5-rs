use crate::crypto::key::KeyAlgorithm;
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
use crate::did::resolver::{DidResolutionError, DidResolutionResult, DidResolver};
use crate::did::Did;
use async_trait::async_trait;
use did_jwk::DIDJWK;
use ssi_dids::did_resolve::DIDResolver;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub struct DidJwkCreateOptions {
    pub key_algorithm: KeyAlgorithm,
}

pub struct DidJwk {
    uri: String,
    key_manager: Arc<dyn KeyManager>,
}

impl Did for DidJwk {
    fn uri(&self) -> &str {
        &self.uri
    }

    fn key_manager(&self) -> &Arc<dyn KeyManager> {
        &self.key_manager
    }
}

impl DidJwk {
    pub fn new(key_manager: Arc<dyn KeyManager>, options: DidJwkCreateOptions) -> Self {
        let key_alias = key_manager
            .generate_private_key(options.key_algorithm)
            .expect("Failed to generate private key");
        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("Failed to get public key")
            .unwrap();

        let uri = DIDJWK
            .generate(&Source::Key(&public_key.inner))
            .expect("DidJwk initialization failed");

        Self { uri, key_manager }
    }
}

#[async_trait]
impl DidResolver for DidJwk {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = did_jwk::DIDJWK
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok(DidResolutionResult {
            did_document,
            did_document_metadata,
            resolution_metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::key_manager::local_key_manager::LocalKeyManager;
    use crate::crypto::key_store::in_memory::InMemoryKeyStore;

    #[test]
    fn test_constructor() {
        let key_store = Arc::new(InMemoryKeyStore::new());
        let key_manager = Arc::new(LocalKeyManager::new(key_store));

        let did = DidJwk::new(
            key_manager,
            DidJwkCreateOptions {
                algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("DidJwk initialization failed");

        assert!(did.uri().starts_with("did:jwk:"));
    }
}
