use crate::crypto::error::CryptoError;
use crate::crypto::key::{KeyAlgorithm, PrivateKey};
use ssi_jwk::JWK;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[uniffi::export]
pub trait KeyStore: Send + Sync {
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, CryptoError>;
    fn insert(&self, value: Arc<PrivateKey>) -> Result<String, CryptoError>;
    fn dump(&self) -> Result<Vec<Arc<PrivateKey>>, CryptoError>;
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

    pub fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> Result<String, CryptoError> {
        let jwk = match key_algorithm {
            KeyAlgorithm::Secp256k1 => {
                JWK::generate_secp256k1().map_err(|_| CryptoError::Unknown)?
            }
            KeyAlgorithm::Secp256r1 => JWK::generate_p256().map_err(|_| CryptoError::Unknown)?,
            KeyAlgorithm::Ed25519 => JWK::generate_ed25519().map_err(|_| CryptoError::Unknown)?,
        };

        let key_alias = self.key_store.insert(Arc::new(PrivateKey(jwk.clone())))?;
        Ok(key_alias)
    }

    pub fn get_public_key(
        &self,
        key_alias: String,
    ) -> Result<Option<Arc<PrivateKey>>, CryptoError> {
        // TODO: Don't love the ending clone. Can/Should get take &str?
        let private_key = self.key_store.get(key_alias.clone())?;
        Ok(private_key)
    }
}

/// A thread-safe in-memory key store.
#[derive(uniffi::Object)]
pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, Arc<PrivateKey>>>,
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
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, CryptoError> {
        Ok(self.map.read().unwrap().get(&key).cloned())
    }

    fn insert(&self, value: Arc<PrivateKey>) -> Result<String, CryptoError> {
        let key = value.0.thumbprint().unwrap();
        self.map.write().unwrap().insert(key.clone(), value);
        println!(
            "map now contains {} entries",
            self.map.read().unwrap().len()
        );
        Ok(key)
    }

    fn dump(&self) -> Result<Vec<Arc<PrivateKey>>, CryptoError> {
        Ok(self.map.read().unwrap().values().cloned().collect())
    }
}
