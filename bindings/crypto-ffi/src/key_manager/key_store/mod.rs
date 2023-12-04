use crate::error::Result;
use crate::key::private_key::PrivateKey;
use crypto::key::private_key::PrivateKey as CryptoPrivateKey;
use crypto::key_manager::key_store::{
    KeyStore as CryptoKeyStore, KeyStoreError as CryptoKeyStoreError,
};
use std::sync::Arc;

pub trait KeyStoreTrait: Send + Sync {
    fn get(&self, key_alias: String) -> Result<Option<Arc<PrivateKey>>>;
    fn insert(&self, key_alias: String, private_key: Arc<PrivateKey>) -> Result<()>;
}

pub struct KeyStore(Arc<dyn KeyStoreTrait>);

impl KeyStore {
    pub fn new(key_store: Arc<dyn KeyStoreTrait>) -> Self {
        Self(key_store)
    }
}

impl CryptoKeyStore for KeyStore {
    fn get(
        &self,
        key_alias: &str,
    ) -> std::result::Result<Option<CryptoPrivateKey>, CryptoKeyStoreError> {
        let private_key = self.0.get(key_alias.to_string())?;
        Ok(private_key.map(|k| k.0.clone()))
    }

    fn insert(
        &self,
        key_alias: &str,
        private_key: CryptoPrivateKey,
    ) -> std::result::Result<(), CryptoKeyStoreError> {
        self.0
            .insert(key_alias.to_string(), Arc::new(private_key.into()))?;
        Ok(())
    }
}
