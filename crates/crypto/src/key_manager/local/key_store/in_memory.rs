use crate::key::PrivateKey;
use crate::key_manager::local::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryKeyStore {
    map: Mutex<HashMap<String, PrivateKey>>,
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        let map = Mutex::new(HashMap::new());
        Self { map }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        let map_lock = self
            .map
            .lock()
            .map_err(|e| KeyStoreError::InternalKeyStoreError {
                message: format!("Unable to acquire Mutex lock: {}", e),
            })?;

        if let Some(private_key) = map_lock.get(key_alias) {
            Ok(Some(private_key.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        let mut map_lock = self
            .map
            .lock()
            .map_err(|e| KeyStoreError::InternalKeyStoreError {
                message: format!("Unable to acquire Mutex lock: {}", e),
            })?;

        map_lock.insert(key_alias.to_string(), private_key);
        Ok(())
    }
}
