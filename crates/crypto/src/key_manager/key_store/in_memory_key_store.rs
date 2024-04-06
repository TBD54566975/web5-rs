use crate::key::private_key::PrivateKey;
use crate::key_manager::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::RwLock;

/// An in-memory implementation of the [`KeyStore`] trait.
pub struct InMemoryKeyStore {
    map: RwLock<HashMap<String, PrivateKey>>,
}

impl Default for InMemoryKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryKeyStore {
    pub fn new() -> Self {
        let map = RwLock::new(HashMap::new());
        Self { map }
    }
}

impl KeyStore for InMemoryKeyStore {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;

        if let Some(private_key) = map_lock.get(key_alias) {
            Ok(Some(private_key.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire Mutex lock: {}", e))
        })?;

        map_lock.insert(key_alias.to_string(), private_key);
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::key::private_key::PrivateKey;
//     use ssi_jwk::JWK;

//     fn new_private_key() -> PrivateKey {
//         PrivateKey(JWK::generate_secp256k1().unwrap())
//     }

//     #[test]
//     fn test_insert_get() {
//         let key_alias = "key-alias";
//         let private_key = new_private_key();

//         let key_store = InMemoryKeyStore::new();
//         key_store.insert(key_alias, private_key.clone()).unwrap();

//         let retrieved_private_key = key_store.get(key_alias).unwrap().unwrap();
//         assert_eq!(private_key, retrieved_private_key);
//     }

//     #[test]
//     fn test_get_missing() {
//         let key_alias = "key-alias";

//         let key_store = InMemoryKeyStore::new();
//         let retrieved_private_key = key_store.get(key_alias).unwrap();
//         assert!(retrieved_private_key.is_none());
//     }
// }
