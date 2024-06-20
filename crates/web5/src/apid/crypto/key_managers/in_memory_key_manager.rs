use super::{key_manager::KeyManager, KeyManagerError, Result};
use crate::apid::{
    crypto::jwk::Jwk,
    dsa::{
        ed25519::{Ed25519Generator, Ed25519Signer},
        Signer,
    },
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct InMemoryKeyManager {
    map: RwLock<HashMap<String, Jwk>>,
}

impl Clone for InMemoryKeyManager {
    fn clone(&self) -> Self {
        let cloned_map = self.map.read().unwrap().clone();
        InMemoryKeyManager {
            map: RwLock::new(cloned_map),
        }
    }
}

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk> {
        let mut public_jwk = private_jwk.clone();
        public_jwk.d = None;

        let mut map_lock = self.map.write().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("unable to acquire Mutex lock: {}", e))
        })?;
        map_lock.insert(public_jwk.compute_thumbprint()?, private_jwk);
        Ok(public_jwk)
    }
}

impl KeyManager for InMemoryKeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let map_lock = self.map.read().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let thumbprint = public_jwk.compute_thumbprint()?;
        let private_jwk = map_lock
            .get(&thumbprint)
            .ok_or(KeyManagerError::KeyNotFound(thumbprint))?;
        Ok(Arc::new(Ed25519Signer::new(private_jwk.clone())))
    }
}
