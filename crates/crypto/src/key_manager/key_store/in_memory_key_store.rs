use crate::key_manager::key_store::{KeyStore, KeyStoreError};
use std::collections::HashMap;
use std::sync::RwLock;
// Assume these traits are defined elsewhere in your crate
use crate::key::{PrivateKey, PublicKey};
use std::marker::PhantomData;

pub struct InMemoryKeyStore<T, U>
where
    T: PrivateKey<U> + Clone + Send + Sync,
    U: PublicKey + Send + Sync,
{
    map: RwLock<HashMap<String, T>>,
    _marker: PhantomData<U>,
}

// Add this to your impl block to initialize _marker
impl<T, U> InMemoryKeyStore<T, U>
where
    T: PrivateKey<U> + Clone + Send + Sync,
    U: PublicKey + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            _marker: PhantomData,
        }
    }
}

impl<T, U> KeyStore<T, U> for InMemoryKeyStore<T, U>
where
    T: PrivateKey<U> + Clone + Send + Sync,
    U: PublicKey + Send + Sync,
{
    fn get(&self, key_alias: &str) -> Result<Option<T>, KeyStoreError> {
        let map_lock = self.map.read().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire read lock: {}", e))
        })?;

        if let Some(private_key) = map_lock.get(key_alias) {
            Ok(Some(private_key.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: T) -> Result<(), KeyStoreError> {
        let mut map_lock = self.map.write().map_err(|e| {
            KeyStoreError::InternalKeyStoreError(format!("Unable to acquire write lock: {}", e))
        })?;

        map_lock.insert(key_alias.to_string(), private_key);
        Ok(())
    }
}

impl<T, U> Default for InMemoryKeyStore<T, U>
where
    T: PrivateKey<U> + Clone + Send + Sync,
    U: PublicKey + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}
