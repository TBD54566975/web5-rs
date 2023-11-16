use crate::crypto::key::PrivateKey;
use crate::crypto::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, PrivateKey>>,
}

#[allow(dead_code)] // TODO: obviously remove this
impl InMemoryKeyStore {
    pub fn new() -> Self {
        let map = RwLock::new(HashMap::new());
        Self { map }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn get(&self, key_alias: &str) -> Result<PrivateKey, KeyStoreError> {
        let readable_map = self
            .map
            .read()
            .map_err(|e| KeyStoreError::UnexpectedReadError {
                message: format!("Unable to acquire RwLockReadGuard: {}", e),
            })?;

        if let Some(private_key) = readable_map.get(key_alias) {
            Ok(private_key.clone())
        } else {
            Err(KeyStoreError::KeyNotFound {
                key_alias: key_alias.to_string(),
            })
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        let mut writable_map =
            self.map
                .write()
                .map_err(|e| KeyStoreError::UnexpectedWriteError {
                    message: format!("Unable to acquire RwLockWriteGuard: {}", e),
                })?;

        writable_map.insert(key_alias.to_string(), private_key);
        Ok(())
    }
}
