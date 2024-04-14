use crate::key::{KeyError, PrivateKey, PublicKey};
use crate::key_manager::key_store::{KeyStore, KeyStoreError};
use crypto::ed25519::Ed25199;
use crypto::secp256k1::Secp256k1;
use crypto::{CryptoError, Curve, CurveOperations, Signer};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, Arc<dyn PrivateKey>>>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn generate_new(&self, curve: Curve) -> Result<String, KeyStoreError> {
        let private_key = Arc::new(match curve {
            Curve::Ed25519 => Ed25199::generate(),
            Curve::Secp256k1 => Secp256k1::generate(),
        }?);
        let key_alias = private_key.compute_thumbprint()?;
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("unable to acquire Mutex lock: {}", e))
        })?;
        map_lock.insert(key_alias.to_string(), private_key);
        Ok(key_alias)
    }

    fn get_all_aliases(&self) -> Result<Vec<String>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let aliases: Vec<String> = map_lock.keys().cloned().collect();
        Ok(aliases)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_key = map_lock
            .get(key_alias)
            .ok_or(KeyStoreError::KeyNotFound(key_alias.to_string()))?;

        let signed_payload = private_key.sign(payload)?;

        Ok(signed_payload)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Arc<dyn PublicKey>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_key = map_lock
            .get(key_alias)
            .ok_or(KeyStoreError::KeyNotFound(key_alias.to_string()))?;
        let public_key = private_key.to_public()?;
        Ok(public_key)
    }

    fn get_signer(&self, key_alias: &str) -> Result<Signer, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_key = map_lock
            .get(key_alias)
            .ok_or(KeyStoreError::KeyNotFound(key_alias.to_string()))?;

        let private_jwk = private_key.jwk()?;

        let signer = match private_jwk.crv.as_str() {
            "Ed25519" => Arc::new(move |payload: &[u8]| -> Result<Vec<u8>, CryptoError> {
                Ed25199::sign(&private_jwk, payload)
            }) as Signer,
            "secp256k1" => Arc::new(move |payload: &[u8]| -> Result<Vec<u8>, CryptoError> {
                Secp256k1::sign(&private_jwk, payload)
            }) as Signer,
            _ => return Err(KeyStoreError::KeyError(KeyError::CurveNotFound)),
        };

        Ok(signer)
    }
}

impl Default for InMemoryKeyStore {
    fn default() -> Self {
        Self::new()
    }
}
