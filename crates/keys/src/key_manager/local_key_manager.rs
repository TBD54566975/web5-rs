use crate::key::PublicKey;
use crate::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crate::key_manager::key_store::KeyStore;
use crate::key_manager::{KeyManager, KeyManagerError};
use crypto::{Curve, Signer};
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
    fn generate_private_key(&self, curve: Curve) -> Result<String, KeyManagerError> {
        let key_alias = self.key_store.generate_new(curve)?;
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

    fn get_signer(&self, key_alias: &str) -> Result<Signer, KeyManagerError> {
        let signer = self.key_store.get_signer(key_alias)?;
        Ok(signer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        key_manager
            .generate_private_key(Curve::Ed25519)
            .expect("Failed to generate Ed25519 key");

        key_manager
            .generate_private_key(Curve::Secp256k1)
            .expect("Failed to generate secp256k1 key");
    }

    #[test]
    fn test_get_public_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        let key_alias = key_manager.generate_private_key(Curve::Ed25519).unwrap();

        key_manager
            .get_public_key(&key_alias)
            .expect("Public key not found");
    }

    #[test]
    fn test_sign() {
        let key_manager = LocalKeyManager::new_in_memory();
        let key_alias = key_manager.generate_private_key(Curve::Ed25519).unwrap();

        // Sign a payload
        let payload: &[u8] = b"hello world";
        let signature = key_manager.sign(&key_alias, payload).unwrap();

        // Get the public key that was used to sign the payload, and verify with it.
        let public_key = key_manager.get_public_key(&key_alias).unwrap();
        assert!(!public_key.verify(payload, &signature).is_err());
    }
}
