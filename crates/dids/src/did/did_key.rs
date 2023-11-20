use crate::did::{Did, DidError};
use crypto::key::{Key, KeyAlgorithm};
use crypto::key_manager::KeyManager;
use did_method_key::DIDKey;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub struct DidKey {
    uri: String,
    key_manager: Arc<dyn KeyManager>,
}

pub struct DidKeyCreateOptions {
    pub key_algorithm: KeyAlgorithm,
}

impl DidKey {
    pub fn new(
        key_manager: Arc<dyn KeyManager>,
        options: DidKeyCreateOptions,
    ) -> Result<Self, DidError> {
        let (_, public_key) = key_manager.generate_private_key(options.key_algorithm)?;

        let uri = DIDKey
            .generate(&Source::Key(&public_key.jwk()))
            .ok_or(DidError::DidCreationFailed)?;

        Ok(Self { uri, key_manager })
    }
}

impl Did for DidKey {
    fn uri(&self) -> &str {
        self.uri.as_ref()
    }

    fn key_manager(&self) -> Arc<dyn KeyManager> {
        self.key_manager.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::key_manager::LocalKeyManager;

    #[tokio::test]
    async fn test_it_works() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let did = DidKey::new(
            key_manager.clone(),
            DidKeyCreateOptions {
                key_algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("Failed to create DidKey");

        let resolve_response = did.resolve().await.expect("Failed to resolve DidKey");
        assert_eq!(resolve_response.did_document.id, did.uri());
    }
}
