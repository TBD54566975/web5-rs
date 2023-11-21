use crate::did::{Did, DidError};
use crypto::key::{Key, KeyAlgorithm};
use crypto::key_manager::KeyManager;
use did_jwk::DIDJWK;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub struct DidJwk {
    uri: String,
    key_manager: Arc<dyn KeyManager>,
}

pub struct DidJwkCreateOptions {
    pub key_algorithm: KeyAlgorithm,
}

impl DidJwk {
    pub fn new(
        key_manager: Arc<dyn KeyManager>,
        options: DidJwkCreateOptions,
    ) -> Result<Self, DidError> {
        let key_alias = key_manager.generate_private_key(options.key_algorithm)?;
        let public_key =
            key_manager
                .get_public_key(&key_alias)?
                .ok_or(DidError::DidCreationFailure(
                    "PublicKey not found".to_string(),
                ))?;

        let uri = DIDJWK.generate(&Source::Key(&public_key.jwk())).ok_or(
            DidError::DidCreationFailure("URI not returned by DIDJWK.generate".to_string()),
        )?;

        Ok(Self { uri, key_manager })
    }
}

impl Did for DidJwk {
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
        let did = DidJwk::new(
            key_manager.clone(),
            DidJwkCreateOptions {
                key_algorithm: KeyAlgorithm::Ed25519,
            },
        )
        .expect("Failed to create DidJwk");

        let resolve_response = did.resolve().await.expect("Failed to resolve DidJwk");
        assert_eq!(resolve_response.did_document.id, did.uri());
    }
}
