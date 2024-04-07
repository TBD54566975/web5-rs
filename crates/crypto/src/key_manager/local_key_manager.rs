use crate::key::private_key::PrivateKey;
use crate::key::public_key::PublicKey;
use crate::key::KeyType;
use crate::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crate::key_manager::key_store::KeyStore;
use crate::key_manager::{KeyManager, KeyManagerError};
use base64::{Engine as _, engine::general_purpose};
use josekit::jwk::{
    alg::{ec::EcCurve, ed::EdCurve},
    Jwk,
};
use sha2::{Digest, Sha256};
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

    /// Constructs a new `LocalKeyManager` that stores keys in memory.
    pub fn new_in_memory() -> Self {
        Self {
            key_store: Arc::new(InMemoryKeyStore::new()),
        }
    }
}

impl KeyManager for LocalKeyManager {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, KeyManagerError> {
        let mut jwk = match key_type {
            KeyType::Secp256k1 => Jwk::generate_ec_key(EcCurve::Secp256k1),
            KeyType::Ed25519 => Jwk::generate_ed_key(EdCurve::Ed25519),
        }?;

        // todo considering proposing adding a thumbprint to the josekit repo
        let thumbprint_json_string = match key_type {
            KeyType::Secp256k1 => format!(
                r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
                jwk.curve().ok_or(KeyManagerError::JoseMissingKeyId)?, // todo different error
                jwk.parameter("x")
                    .ok_or(KeyManagerError::JoseMissingKeyId)?,
                jwk.parameter("y")
                    .ok_or(KeyManagerError::JoseMissingKeyId)?,
            ),
            KeyType::Ed25519 => format!(
                r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#,
                jwk.curve().ok_or(KeyManagerError::JoseMissingKeyId)?, // todo different error
                jwk.parameter("x")
                    .ok_or(KeyManagerError::JoseMissingKeyId)?, // todo different error
            ),
        };
        let mut hasher = Sha256::new();
        hasher.update(thumbprint_json_string);
        let digest = hasher.finalize();
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(&digest);
        let key_alias = thumbprint.clone();

        jwk.set_key_id(&key_alias);

        let private_key = PrivateKey(jwk);

        self.key_store.insert(&key_alias, private_key)?;

        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
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

    fn alias(&self, public_key: &PublicKey) -> Result<String, KeyManagerError> {
        let key_id = public_key
            .0
            .key_id()
            .ok_or(KeyManagerError::JoseMissingKeyId)?;
        Ok(key_id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_private_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        key_manager
            .generate_private_key(KeyType::Ed25519)
            .expect("Failed to generate Ed25519 key");

        key_manager
            .generate_private_key(KeyType::Secp256k1)
            .expect("Failed to generate secp256k1 key");
    }

    #[test]
    fn test_get_public_key() {
        let key_manager = LocalKeyManager::new_in_memory();

        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();

        key_manager
            .get_public_key(&key_alias)
            .unwrap()
            .expect("Public key not found");
    }

    #[test]
    fn test_sign() {
        let key_manager = LocalKeyManager::new_in_memory();
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
