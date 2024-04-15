use crate::key::{PrivateKey, PublicKey};
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
    fn generate_new(&self, curve: Curve, key_alias: Option<&str>) -> Result<String, KeyStoreError> {
        let private_key = Arc::new(match curve {
            Curve::Ed25519 => Ed25199::generate(),
            Curve::Secp256k1 => Secp256k1::generate(),
        }?);
        let key_alias = match key_alias {
            Some(key_alias) => key_alias.to_string(),
            None => private_key.compute_thumbprint()?,
        };
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("unable to acquire Mutex lock: {}", e))
        })?;
        map_lock.insert(key_alias.clone(), private_key);
        Ok(key_alias)
    }

    fn get_all_aliases(&self) -> Result<Vec<String>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let aliases = map_lock.keys().cloned().collect();
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
        let signer = Arc::new(move |payload: &[u8]| -> Result<Vec<u8>, CryptoError> {
            private_jwk
                .sign(payload)
                .map_err(|e| CryptoError::SignFailure(e.to_string()))
        });
        Ok(signer)
    }

    fn export_private_keys(&self) -> Result<Vec<Arc<dyn PrivateKey>>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let private_keys = map_lock.values().cloned().collect();
        Ok(private_keys)
    }

    fn import_private_keys(
        &self,
        private_keys: Vec<Arc<dyn PrivateKey>>,
    ) -> Result<(), KeyStoreError> {
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;

        for key in private_keys {
            let key_alias = key.alias()?;
            map_lock.insert(key_alias, key);
        }

        Ok(())
    }
}

impl Default for InMemoryKeyStore {
    fn default() -> Self {
        Self::new()
    }
}
