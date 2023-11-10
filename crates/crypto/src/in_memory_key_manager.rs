use crate::{generate_private_key, Key, KeyAlgorithm, KeyManager, KeyManagerError, PrivateKey};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryKeyManager {
    key_store: Mutex<HashMap<String, PrivateKey>>,
}

impl InMemoryKeyManager {
    fn new() -> Self {
        Self {
            key_store: Mutex::new(HashMap::new()),
        }
    }
}

impl KeyManager for InMemoryKeyManager {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyManagerError> {
        let private_key = generate_private_key(key_algorithm);
        let key_alias = private_key.alias();

        let mut key_store = self.key_store.lock().unwrap();
        key_store.insert(key_alias.clone(), private_key.clone());

        Ok(private_key.into())
    }
}
