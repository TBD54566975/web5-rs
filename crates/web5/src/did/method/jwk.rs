use crate::crypto::key::{Key, KeyAlgorithm};
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
use crate::did::resolver::{DidResolutionError, DidResolutionResponse, DidResolver};
use crate::did::Did;
use crate::result::Web5Result;
use async_trait::async_trait;
use did_jwk::DIDJWK;
use ssi_dids::did_resolve::DIDResolver;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub struct DidJwkCreateOptions {
    pub key_algorithm: KeyAlgorithm,
}

pub struct DidJwkData {}

pub type DidJwk = Did<DidJwkData>;

impl DidJwk {
    pub fn new(key_manager: Arc<dyn KeyManager>, options: DidJwkCreateOptions) -> Web5Result<Self> {
        let (_, public_key) = key_manager.generate_private_key(options.key_algorithm)?;

        let uri = DIDJWK
            .generate(&Source::Key(&public_key.jwk()))
            .expect("DidJwk initialization failed");

        let method_data = DidJwkData {};

        Ok(Self {
            uri,
            key_manager,
            method_data,
        })
    }
}

#[async_trait]
impl DidResolver for DidJwk {
    async fn resolve(did_uri: &str) -> Result<DidResolutionResponse, DidResolutionError> {
        let (resolution_metadata, did_document, did_document_metadata) = did_jwk::DIDJWK
            .resolve(
                did_uri,
                &ssi_dids::did_resolve::ResolutionInputMetadata::default(),
            )
            .await;

        let did_document = did_document.ok_or(DidResolutionError::DidDocumentNotFound)?;

        Ok((resolution_metadata, did_document, did_document_metadata))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::key_manager::local::key_store::in_memory::InMemoryKeyStore;
    use crate::crypto::key_manager::local::LocalKeyManager;
    #[test]
    fn test_constructor() {
        let key_store = Arc::new(InMemoryKeyStore::new());
        let key_manager = Arc::new(LocalKeyManager::new(key_store));

        let did = DidJwk::new(
            key_manager,
            DidJwkCreateOptions {
                key_algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("DidJwk initialization failed");

        assert!(did.uri.starts_with("did:jwk:"));
    }
}
