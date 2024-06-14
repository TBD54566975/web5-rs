use super::{
    dsa::ed25519::{Ed25519Generator, Ed25519Signer},
    jwk::{Jwk, JwkError},
};
use std::{collections::HashMap, sync::RwLock};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyManagerError {
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("{0}")]
    InternalKeyStoreError(String),
    #[error("key not found {0}")]
    KeyNotFound(String),
}

type Result<T> = std::result::Result<T, KeyManagerError>;

pub struct InMemoryKeyManager {
    map: RwLock<HashMap<String, Jwk>>,
}

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn generate_key_material(&self) -> Result<Jwk> {
        let private_jwk = Ed25519Generator::generate();
        let public_jwk = self.import_key(private_jwk)?;
        Ok(public_jwk)
    }

    pub fn get_signer(&self, public_jwk: Jwk) -> Result<Ed25519Signer> {
        let map_lock = self.map.read().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;
        let thumbprint = public_jwk.compute_thumbprint()?;
        let private_jwk = map_lock
            .get(&thumbprint)
            .ok_or(KeyManagerError::KeyNotFound(thumbprint))?;
        Ok(Ed25519Signer::new(private_jwk.clone()))
    }

    pub fn import_key(&self, private_jwk: Jwk) -> Result<Jwk> {
        let mut public_jwk = private_jwk.clone();
        public_jwk.d = None;

        let mut map_lock = self.map.write().map_err(|e| {
            KeyManagerError::InternalKeyStoreError(format!("unable to acquire Mutex lock: {}", e))
        })?;
        map_lock.insert(public_jwk.compute_thumbprint()?, private_jwk);
        Ok(public_jwk)
    }
}
