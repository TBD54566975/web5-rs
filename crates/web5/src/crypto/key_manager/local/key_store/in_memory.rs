use crate::crypto::key::PrivateKey;
use crate::crypto::key_manager::local::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, PrivateKey>>,
}

#[allow(dead_code)] // TODO: obviously remove this
impl InMemoryKeyStore {
    pub fn new() -> Self {
        println!("Making a new InMemoryKeyStore!");
        let map = RwLock::new(HashMap::new());
        Self { map }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        let readable_map = self
            .map
            .read()
            .map_err(|e| KeyStoreError::InternalKeyStoreError {
                message: format!(
                    "InMemoryKeyStore - Unable to acquire RwLockReadGuard: {}",
                    e
                ),
            })?;

        if let Some(private_key) = readable_map.get(key_alias) {
            Ok(Some(private_key.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        let mut writable_map =
            self.map
                .write()
                .map_err(|e| KeyStoreError::InternalKeyStoreError {
                    message: format!(
                        "InMemoryKeyStore - Unable to acquire RwLockWriteGuard: {}",
                        e
                    ),
                })?;

        writable_map.insert(key_alias.to_string(), private_key);
        println!("InMemory inserted private_key: {}", key_alias);
        Ok(())
    }
}
