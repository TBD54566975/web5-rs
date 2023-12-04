pub mod key_store;

use crate::error::Result;
use crate::key::public_key::PublicKey;
use crypto::key::public_key::PublicKey as CryptoPublicKey;
use crypto::key::KeyType;
use crypto::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crypto::key_manager::local_key_manager::LocalKeyManager;
use crypto::key_manager::{
    KeyManager as CryptoKeyManager, KeyManagerError as CryptoKeyManagerError,
};
use std::sync::Arc;

pub trait KeyManagerTrait: Send + Sync {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String>;
    fn get_public_key(&self, key_alias: String) -> Result<Option<Arc<PublicKey>>>;
    fn sign(&self, key_alias: String, payload: Vec<u8>) -> Result<Vec<u8>>;
    fn alias(&self, public_key: Arc<PublicKey>) -> Result<String>;
}

pub struct KeyManager(Arc<dyn KeyManagerTrait>);

impl KeyManager {
    pub fn new(key_manager: Arc<dyn KeyManagerTrait>) -> Self {
        Self(key_manager)
    }

    pub fn in_memory() -> Self {
        let key_manager = LocalKeyManager::new(Arc::new(InMemoryKeyStore::new()));
        Self(Arc::new(key_manager))
    }
}

impl CryptoKeyManager for KeyManager {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, CryptoKeyManagerError> {
        Ok(self.0.generate_private_key(key_type)?)
    }

    fn get_public_key(
        &self,
        key_alias: &str,
    ) -> Result<Option<CryptoPublicKey>, CryptoKeyManagerError> {
        let public_key = self.0.get_public_key(key_alias.to_string())?;
        Ok(public_key.map(|k| k.0.clone()))
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, CryptoKeyManagerError> {
        Ok(self.0.sign(key_alias.to_string(), payload.to_vec())?)
    }

    fn alias(&self, public_key: &CryptoPublicKey) -> Result<String, CryptoKeyManagerError> {
        let public_key = PublicKey(public_key.clone());
        Ok(self.0.alias(Arc::new(public_key))?)
    }
}

impl<T: CryptoKeyManager> KeyManagerTrait for T {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String> {
        let key_alias = self.generate_private_key(key_type)?;
        Ok(key_alias)
    }

    fn get_public_key(&self, key_alias: String) -> Result<Option<Arc<PublicKey>>> {
        let public_key = self.get_public_key(&key_alias)?;
        Ok(public_key.map(|k| Arc::new(PublicKey(k))))
    }

    fn sign(&self, key_alias: String, payload: Vec<u8>) -> Result<Vec<u8>> {
        let signed_payload = self.sign(&key_alias, &payload)?;
        Ok(signed_payload)
    }

    fn alias(&self, public_key: Arc<PublicKey>) -> Result<String> {
        let alias = self.alias(&public_key.0)?;
        Ok(alias)
    }
}
