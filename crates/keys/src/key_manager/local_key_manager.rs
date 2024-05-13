use crate::key::{PrivateKey, PublicKey};
use crate::key_manager::{KeyManager, KeyManagerError};
use crypto::ed25519::Ed25519;
use crypto::secp256k1::Secp256k1;
use crypto::{Curve, CurveOperations};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::KeyImporter;

/// Implementation of the [`KeyManager`] trait with key generation local to the device/platform it
/// is being run. Key storage is provided by a [`KeyStore`] trait implementation, allowing the keys
/// to be stored wherever is most appropriate for the application.
pub struct LocalKeyManager {
    map: RwLock<HashMap<String, Arc<dyn PrivateKey>>>,
}

impl LocalKeyManager {
    /// Constructs a new `LocalKeyManager` that stores keys in memory.
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl KeyManager for LocalKeyManager {
    fn generate_private_key(
        &self,
        curve: Curve,
        key_alias: Option<String>,
    ) -> Result<String, KeyManagerError> {
        let private_key = Arc::new(match curve {
            Curve::Ed25519 => Ed25519::generate(),
            Curve::Secp256k1 => Secp256k1::generate(),
        }?);
        let key_alias = match key_alias {
            Some(key_alias) => key_alias,
            None => private_key.compute_thumbprint()?,
        };
        let mut map_lock = self.map.write().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("unable to acquire Mutex lock: {}", e))
        })?;
        map_lock.insert(key_alias.clone(), private_key);
        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyManagerError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_key = map_lock
            .get(key_alias)
            .ok_or(KeyManagerError::KeyNotFound(key_alias.to_string()))?;
        let public_key = private_key.to_public()?;
        Ok(public_key)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        let map_lock: std::sync::RwLockReadGuard<HashMap<String, Arc<dyn PrivateKey>>> = self.map.read().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_key = map_lock
            .get(key_alias)
            .ok_or(KeyManagerError::KeyNotFound(key_alias.to_string()))?;

        let signed_payload = private_key.sign(payload)?;

        Ok(signed_payload)
    }
}

impl KeyImporter for LocalKeyManager {
    fn import_with_alias(
        &self,
        private_key: Arc<dyn PrivateKey>,
        key_alias: &str,
    ) -> Result<(), KeyManagerError> {
        let mut map_lock = self.map.write().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;

        map_lock.insert(key_alias.to_owned(), private_key);

        Ok(())
    }
}



impl Default for LocalKeyManager {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use crate::key::Key;

    use super::*;

    #[test]
    fn test_generate_private_key() {
        let key_manager = LocalKeyManager::new();

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
        let key_manager = LocalKeyManager::new();

        let key_alias = key_manager
            .generate_private_key(Curve::Ed25519, None)
            .unwrap();

        key_manager
            .get_public_key(&key_alias)
            .expect("Public key not found");
    }

    #[test]
    fn test_sign() {
        let key_manager: LocalKeyManager = LocalKeyManager::new();
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

    #[test]
    fn test_import() {
        let key_manager = LocalKeyManager::new();
        let private_key = Arc::new(Ed25519::generate().unwrap());

        let key_alias = key_manager.import(private_key.clone()).expect("Failed to import private key");
        let default_alias = private_key.alias().expect("Failed to generate private key alias");
        assert_eq!(key_alias, default_alias);

        let key_manager_public_key = key_manager.get_public_key(&key_alias)
            .expect("Failed to get public key")
            .jwk()
            .expect("Failed to get alias of public key");
        let public_key = private_key.to_public()
            .expect("Failed to convert private key to public")
            .jwk()
            .expect("Failed to get alias of public key");
        assert_eq!(public_key, key_manager_public_key)
    }

    #[test]
    fn test_import_with_alias() {
        let key_manager = LocalKeyManager::new();
        let private_key = Arc::new(Ed25519::generate().unwrap());

        let key_alias = "1234".to_string();
        key_manager.import_with_alias(private_key.clone(), &key_alias)
            .expect("Failed to import private key with alias");

        let key_manager_public_key = key_manager.get_public_key(&key_alias)
            .expect("Failed to get public key")
            .jwk()
            .expect("Failed to get alias of public key");
        let public_key = private_key.to_public()
            .expect("Failed to convert private key to public")
            .jwk()
            .expect("Failed to get alias of public key");
        assert_eq!(public_key, key_manager_public_key)
    }
}
