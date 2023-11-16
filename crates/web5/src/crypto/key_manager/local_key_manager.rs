use crate::crypto::key::{Key, KeyError};
use crate::crypto::key::{KeyAlgorithm, PrivateKey, PublicKey};
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
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
    fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, KeyManagerError> {
        let private_key = PrivateKey::new(key_algorithm).map_err(|e| KeyManagerError::Generic {
            message: e.to_string(),
        })?;
        let key_alias = private_key.alias().map_err(|e| KeyManagerError::Generic {
            message: e.to_string(),
        })?;

        self.key_store
            .insert(&key_alias, private_key)
            .map_err(|e| KeyManagerError::Generic {
                message: e.to_string(),
            })?;
        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
        if let Some(private_key) =
            self.key_store
                .get(key_alias)
                .map_err(|e| KeyManagerError::Generic {
                    message: e.to_string(),
                })?
        {
            Ok(Some(PublicKey::from(private_key)))
        } else {
            Ok(None)
        }
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let private_key = self
            .key_store
            .get(key_alias)
            .map_err(|e| KeyManagerError::Generic {
                message: e.to_string(),
            })?;

        let private_key = private_key.ok_or(KeyManagerError::Generic {
            message: "Key not found".to_string(),
        })?;

        let signed_payload =
            private_key
                .sign(&payload.to_vec())
                .map_err(|e| KeyManagerError::Generic {
                    message: e.to_string(),
                })?;

        Ok(signed_payload)
    }

    fn get_deterministic_alias(&self, public_key: PublicKey) -> Result<String, KeyManagerError> {
        Ok(public_key.alias().map_err(|e| KeyManagerError::Generic {
            message: e.to_string(),
        })?)
    }
}
