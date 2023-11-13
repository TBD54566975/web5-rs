use crate::crypto::key::{KeyAlgorithm, PrivateKey, PublicKey};
use crate::error::Web5Error;
use ssi_jwk::JWK;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[uniffi::export]
pub trait KeyStore: Send + Sync {
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, Web5Error>;
    fn insert(&self, value: Arc<PrivateKey>) -> Result<String, Web5Error>;
    fn dump(&self) -> Result<Vec<Arc<PrivateKey>>, Web5Error>;
}

#[derive(uniffi::Object)]
pub struct KeyManager {
    key_store: Arc<dyn KeyStore>,
}

#[uniffi::export]
impl KeyManager {
    #[uniffi::constructor]
    pub fn new(key_store: Arc<dyn KeyStore>) -> Arc<Self> {
        Arc::new(Self { key_store })
    }

    pub fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, Web5Error> {
        let jwk = match key_algorithm {
            KeyAlgorithm::Secp256k1 => JWK::generate_secp256k1().map_err(|_| Web5Error::Unknown)?,
            KeyAlgorithm::Secp256r1 => JWK::generate_p256().map_err(|_| Web5Error::Unknown)?,
            KeyAlgorithm::Ed25519 => JWK::generate_ed25519().map_err(|_| Web5Error::Unknown)?,
        };

        let key_alias = self.key_store.insert(Arc::new(PrivateKey(jwk.clone())))?;
        Ok(key_alias)
    }

    pub fn get_public_key(&self, key_alias: String) -> Result<Option<Arc<PublicKey>>, Web5Error> {
        // TODO: Don't love the ending clone. Can/Should get take &str?
        let private_key = self
            .key_store
            .get(key_alias.clone())?
            .expect("Couldn't generate public key");

        Ok(Some(private_key.to_public_key()))
    }

    pub fn sign(&self, key_alias: String, payload: &Vec<u8>) -> Vec<u8> {
        // TODO: It goes without saying: unwrapping is bad, and I need to go through
        // all the code so far and remove them with proper error handling.
        let private_key = self.key_store.get(key_alias).unwrap().unwrap();
        private_key.sign(&payload)
    }

    fn get_key_store(&self) -> Arc<dyn KeyStore> {
        self.key_store.clone()
    }
}

/// A thread-safe in-memory key store.
#[derive(uniffi::Object)]
pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, Arc<PrivateKey>>>,
}

impl Drop for InMemoryKeyStore {
    fn drop(&mut self) {
        println!("InMemoryKeyStore deallocated!");
    }
}

#[uniffi::export]
impl InMemoryKeyStore {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            map: RwLock::new(HashMap::new()),
        })
    }
}

#[uniffi::export]
impl KeyStore for InMemoryKeyStore {
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, Web5Error> {
        Ok(self.map.read().unwrap().get(&key).cloned())
    }

    fn insert(&self, value: Arc<PrivateKey>) -> Result<String, Web5Error> {
        let key = value.0.thumbprint().unwrap();
        self.map.write().unwrap().insert(key.clone(), value);
        Ok(key)
    }

    fn dump(&self) -> Result<Vec<Arc<PrivateKey>>, Web5Error> {
        Ok(self.map.read().unwrap().values().cloned().collect())
    }
}
