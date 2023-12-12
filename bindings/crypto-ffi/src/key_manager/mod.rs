pub mod key_store;

use crate::error::Result;
use crate::key::public_key::PublicKeyFfi;
use crate::key_manager::key_store::{KeyStoreFfi, KeyStoreFfiAdapter};
use crypto::key::public_key::PublicKey;
use crypto::key::KeyType;
use crypto::key_manager::local_key_manager::LocalKeyManager;
use crypto::key_manager::{KeyManager, KeyManagerError};
use std::sync::Arc;

pub struct KeyManagerFfi(Box<dyn KeyManager>);

impl KeyManagerFfi {
    pub fn new_in_memory() -> Self {
        Self(Box::new(LocalKeyManager::new_in_memory()))
    }

    pub fn new(key_store: Arc<dyn KeyStoreFfi>) -> Self {
        Self(Box::new(LocalKeyManager::new(Arc::new(
            KeyStoreFfiAdapter(key_store),
        ))))
    }

    pub fn generate_private_key(&self, key_type: KeyType) -> Result<String> {
        Ok(self.0.generate_private_key(key_type)?)
    }

    pub fn get_public_key(&self, key_alias: &str) -> Result<Option<Arc<PublicKeyFfi>>> {
        Ok(self.0.get_public_key(key_alias).map(|public_key| {
            public_key.map(|public_key| Arc::new(PublicKeyFfi::from(public_key)))
        })?)
    }

    pub fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>> {
        Ok(self.0.sign(key_alias, payload)?)
    }

    pub fn alias(&self, public_key: &PublicKeyFfi) -> Result<String> {
        Ok(self.0.alias(&public_key.0)?)
    }
}

impl KeyManager for KeyManagerFfi {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, KeyManagerError> {
        self.0.generate_private_key(key_type)
    }

    fn get_public_key(&self, key_alias: &str) -> Result<Option<PublicKey>, KeyManagerError> {
        self.0.get_public_key(key_alias)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        self.0.sign(key_alias, payload)
    }

    fn alias(&self, public_key: &PublicKey) -> Result<String, KeyManagerError> {
        self.0.alias(public_key)
    }
}
