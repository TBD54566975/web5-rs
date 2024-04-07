use crate::key::jwk::private_jwk::PrivateJwk;
use crate::key::jwk::public_jwk::PublicJwk;
use crate::key::{KeyType, PrivateKey, PublicKey};
use crate::key_manager::key_store::in_memory_jwk_store::InMemoryJwkStore;
use crate::key_manager::key_store::KeyStore;
use crate::key_manager::{KeyManager, KeyManagerError};
use std::sync::Arc;

/// Implementation of the [`KeyManager`] trait with key generation local to the device/platform it
/// is being run. Key storage is provided by a [`KeyStore`] trait implementation, allowing the keys
/// to be stored wherever is most appropriate for the application.
pub struct LocalJwkManager {
    key_store: Arc<dyn KeyStore<PrivateJwk, PublicJwk>>,
}

impl LocalJwkManager {
    /// Constructs a new `LocalKeyManager` that stores keys in the provided `KeyStore`.
    pub fn new(key_store: Arc<dyn KeyStore<PrivateJwk, PublicJwk>>) -> Self {
        Self { key_store }
    }

    /// Constructs a new `LocalKeyManager` that stores keys in memory.
    pub fn new_in_memory() -> Self {
        Self {
            key_store: Arc::new(InMemoryJwkStore::new()),
        }
    }
}

impl KeyManager<PublicJwk> for LocalJwkManager {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, KeyManagerError> {
        let private_key =
            PrivateJwk::generate(key_type).map_err(|_| KeyManagerError::KeyGenerationFailed)?;
        let public_key = private_key
            .to_public()
            .map_err(|_| KeyManagerError::KeyGenerationFailed)?;
        let key_alias = self.alias(&public_key)?;

        self.key_store.insert(&key_alias, private_key)?;

        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicJwk>, KeyManagerError> {
        if let Some(private_key) = self.key_store.get(key_alias)? {
            let public_key = private_key.to_public()?;
            Ok(Some(public_key))
        } else {
            Ok(None)
        }
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let private_key = self
            .key_store
            .get(key_alias)?
            .ok_or(KeyManagerError::SigningKeyNotFound)?;

        let signed_payload = private_key.sign(payload)?;

        Ok(signed_payload)
    }

    fn alias(&self, public_key: &PublicJwk) -> Result<String, KeyManagerError> {
        let alias = public_key.alias().map_err(KeyManagerError::KeyError)?;
        Ok(alias)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let key_manager = LocalJwkManager::new_in_memory();

        key_manager
            .generate_private_key(KeyType::Ed25519)
            .expect("Failed to generate Ed25519 key");

        key_manager
            .generate_private_key(KeyType::Secp256k1)
            .expect("Failed to generate secp256k1 key");
    }

    #[test]
    fn test_get_public_key() {
        let key_manager = LocalJwkManager::new_in_memory();

        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();

        key_manager
            .get_public_key(&key_alias)
            .unwrap()
            .expect("Public key not found");
    }

    #[test]
    fn test_sign() {
        let key_manager = LocalJwkManager::new_in_memory();
        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();

        // Sign a payload
        let payload: &[u8] = b"hello world";
        let signature = key_manager.sign(&key_alias, payload).unwrap();

        // Get the public key that was used to sign the payload, and verify with it.
        let public_key = key_manager.get_public_key(&key_alias).unwrap().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }

    // #[test]
    // fn test_alias() {
    //     let key_manager = LocalKeyManager::new_in_memory();
    //     let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();

    //     let public_key = key_manager.get_public_key(&key_alias).unwrap().unwrap();
    //     let alias = key_manager.alias(&public_key).unwrap();

    //     assert_eq!(key_alias, alias);
    // }
}