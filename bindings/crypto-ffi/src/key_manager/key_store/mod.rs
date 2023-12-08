pub mod custom_key_store;

use crypto::key::private_key::PrivateKey as CryptoPrivateKey;
use crypto::key_manager::key_store::in_memory_key_store::InMemoryKeyStore;
use crypto::key_manager::key_store::{
    KeyStore as CryptoKeyStore, KeyStoreError as CryptoKeyStoreError,
};
use custom_key_store::{CustomKeyStore, CustomKeyStoreAdapter};
use std::sync::Arc;

/// Concrete `KeyStore` struct, exposed to foreign languages.
///
/// This struct is a new type around the [`crypto::key_manager::key_store::KeyStore`] trait, which
/// is not exposed to foreign languages because it is incompatible with FFI. This struct allows
/// foreign languages to create and manage their own instance of a KeyStore, without the need of
/// exposing the underlying trait directly.
pub struct KeyStore(pub(crate) Box<dyn CryptoKeyStore>);

impl KeyStore {
    pub fn new(custom: Arc<dyn CustomKeyStore>) -> Self {
        Self(Box::new(CustomKeyStoreAdapter(custom)))
    }

    pub fn new_in_memory() -> Self {
        Self(Box::new(InMemoryKeyStore::new()))
    }
}

impl CryptoKeyStore for KeyStore {
    fn get(&self, key_alias: &str) -> Result<Option<CryptoPrivateKey>, CryptoKeyStoreError> {
        Ok(self.0.get(key_alias)?)
    }

    fn insert(
        &self,
        key_alias: &str,
        private_key: CryptoPrivateKey,
    ) -> Result<(), CryptoKeyStoreError> {
        Ok(self.0.insert(key_alias, private_key)?)
    }
}
