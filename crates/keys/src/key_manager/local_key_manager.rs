use crate::key::{PrivateKey, PublicKey};
use crate::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crate::key_manager::key_store::KeyStore;
use crate::key_manager::{KeyManager, KeyManagerError};
use crypto::Curve;
use std::sync::Arc;

/// Implementation of the [`KeyManager`] trait with key generation local to the device/platform it
/// is being run. Key storage is provided by a [`KeyStore`] trait implementation, allowing the keys
/// to be stored wherever is most appropriate for the application.
pub struct LocalKeyManager {
    key_store: Arc<dyn KeyStore>,
}

impl LocalKeyManager {
    /// Constructs a new `LocalKeyManager` that stores keys in the provided `KeyStore`.
    pub fn new(key_store: Arc<dyn KeyStore>) -> Self {
        Self { key_store }
    }

    pub fn new_in_memory() -> Self {
        Self {
            key_store: Arc::new(InMemoryKeyStore::new()),
        }
    }
}

impl KeyManager for LocalKeyManager {
    fn generate_private_key(
        &self,
        curve: Curve,
        key_alias: Option<String>,
    ) -> Result<String, KeyManagerError> {
        let key_alias = self
            .key_store
            .generate_new(curve, key_alias.as_ref().map(|x| x.as_str()))?;
        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyManagerError> {
        let public_key = self.key_store.get_public_key(key_alias)?;
        Ok(public_key)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let signed_payload = self.key_store.sign(key_alias, payload)?;
        Ok(signed_payload)
    }

    fn export_private_keys(&self) -> Result<Vec<Arc<dyn PrivateKey>>, KeyManagerError> {
        let private_keys = self.key_store.export_private_keys()?;
        Ok(private_keys)
    }

    fn import_private_keys(
        &self,
        private_keys: Vec<Arc<dyn PrivateKey>>,
    ) -> Result<(), KeyManagerError> {
        self.key_store.import_private_keys(private_keys)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        key_manager
            .generate_private_key(Curve::Ed25519, None)
            .expect("Failed to generate Ed25519 key");

        key_manager
            .generate_private_key(Curve::Secp256k1, None)
            .expect("Failed to generate secp256k1 key");

        let key_alias_override = Some("key-id-123".to_string());
        let key_alias = key_manager
            .generate_private_key(Curve::Secp256k1, key_alias_override.clone())
            .expect("Failed to generate secp256k1 key");
        assert_eq!(key_alias_override.unwrap().to_string(), key_alias)
    }

    #[test]
    fn test_get_public_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        let key_alias = key_manager
            .generate_private_key(Curve::Ed25519, None)
            .unwrap();

        key_manager
            .get_public_key(&key_alias)
            .expect("Public key not found");
    }

    #[test]
    fn test_sign() {
        let key_manager = LocalKeyManager::new_in_memory();
        let key_alias = key_manager
            .generate_private_key(Curve::Ed25519, None)
            .unwrap();

        // Sign a payload
        let payload: &[u8] = b"hello world";
        let signature = key_manager.sign(&key_alias, payload).unwrap();

        // Get the public key that was used to sign the payload, and verify with it.
        let public_key = key_manager.get_public_key(&key_alias).unwrap();
        assert!(!public_key.verify(payload, &signature).is_err());
    }
}
