pub mod custom_key_store;

use crypto::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crypto::key_manager::key_store::KeyStore as CryptoKeyStore;
use custom_key_store::{CustomKeyStore, CustomKeyStoreAdapter};
use std::sync::Arc;

pub struct KeyStore(pub(crate) Box<dyn CryptoKeyStore>);

impl KeyStore {
    pub fn new(custom_key_store: Arc<dyn CustomKeyStore>) -> Self {
        Self(Box::new(CustomKeyStoreAdapter(custom_key_store)))
    }

    pub fn in_memory() -> Self {
        Self(Box::new(InMemoryKeyStore::new()))
    }
}

impl CryptoKeyStore for KeyStore {
    fn get(
        &self,
        key_alias: &str,
    ) -> Result<
        Option<crypto::key::private_key::PrivateKey>,
        crypto::key_manager::key_store::KeyStoreError,
    > {
        Ok(self.0.get(key_alias)?)
    }

    fn insert(
        &self,
        key_alias: &str,
        private_key: crypto::key::private_key::PrivateKey,
    ) -> Result<(), crypto::key_manager::key_store::KeyStoreError> {
        Ok(self.0.insert(key_alias, private_key)?)
    }
}
