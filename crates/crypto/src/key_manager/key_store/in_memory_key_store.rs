use crate::key::PrivateKey;
use crate::key_manager::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, Box<dyn PrivateKey>>>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn get(&self, key_alias: &str) -> Result<Option<Box<dyn PrivateKey>>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire read lock: {}", e))
        })?;

        if let Some(private_key) = map_lock.get(key_alias) {
            Ok(Some(private_key.clone_box()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: Box<dyn PrivateKey>) -> Result<(), KeyStoreError> {
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire write lock: {}", e))
        })?;

        map_lock.insert(key_alias.to_string(), private_key);
        Ok(())
    }
}

impl Default for InMemoryKeyStore {
    fn default() -> Self {
        Self::new()
    }
}
