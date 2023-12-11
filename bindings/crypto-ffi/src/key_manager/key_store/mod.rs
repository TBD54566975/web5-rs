pub mod custom_key_store;

use crypto::key::private_key::PrivateKey;
use crypto::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crypto::key_manager::key_store::{KeyStore, KeyStoreError};

pub struct KeyStoreFfi(pub(crate) Box<dyn KeyStore>);

impl KeyStoreFfi {
    pub fn new_in_memory() -> Self {
        Self(Box::new(InMemoryKeyStore::new()))
    }
}

impl KeyStore for KeyStoreFfi {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        Ok(self.0.get(key_alias)?)
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        Ok(self.0.insert(key_alias, private_key)?)
    }
}
