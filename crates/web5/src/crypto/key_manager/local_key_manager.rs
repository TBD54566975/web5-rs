use crate::crypto::key::{Key, KeyAlgorithm, PrivateKey, PublicKey};
use crate::crypto::key_manager::{GeneratePrivateKeyResponse, KeyManager, KeyManagerError};
use crate::crypto::key_store::KeyStore;
use std::sync::Arc;

pub struct LocalKeyManager {
    key_store: Arc<dyn KeyStore>,
}

impl LocalKeyManager {
    pub fn new(key_store: Arc<dyn KeyStore>) -> Self {
        Self { key_store }
    }
}

impl KeyManager for LocalKeyManager {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<GeneratePrivateKeyResponse, KeyManagerError> {
        let private_key = PrivateKey::generate(key_algorithm)?;
        let key_alias = private_key.alias()?;
        let public_key = private_key.to_public();

        self.key_store.insert(&key_alias, private_key)?;
        Ok(GeneratePrivateKeyResponse {
            key_alias,
            public_key,
        })
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
        if let Some(private_key) = self.key_store.get(key_alias)? {
            Ok(Some(PublicKey::from(private_key)))
        } else {
            Ok(None)
        }
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let private_key = self
            .key_store
            .get(key_alias)?
            .ok_or(KeyManagerError::SigningKeyNotFound)?;

        let signed_payload = private_key.sign(&payload.to_vec())?;

        Ok(signed_payload)
    }

    fn get_deterministic_alias(&self, public_key: PublicKey) -> Result<String, KeyManagerError> {
        Ok(public_key.alias()?)
    }
}
