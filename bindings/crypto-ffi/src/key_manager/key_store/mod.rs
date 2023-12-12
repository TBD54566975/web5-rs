use crate::error::Result;
use crate::key::private_key::PrivateKeyFfi;
use crypto::key::private_key::PrivateKey;
use crypto::key_manager::key_store::{KeyStore, KeyStoreError};
use std::sync::Arc;

pub trait KeyStoreFfi: Send + Sync {
    fn get(&self, key_alias: String) -> Result<Option<Arc<PrivateKeyFfi>>>;
    fn insert(&self, key_alias: String, private_key: Arc<PrivateKeyFfi>) -> Result<()>; // TODO: does this need to be an Arc<PrivateKeyFfi>?
}

pub struct KeyStoreFfiAdapter(pub(crate) Arc<dyn KeyStoreFfi>);

impl KeyStore for KeyStoreFfiAdapter {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        Ok(self
            .0
            .get(key_alias.to_string())?
            .map(|private_key| private_key.0.clone()))
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        Ok(self.0.insert(
            key_alias.to_string(),
            Arc::new(PrivateKeyFfi::from(private_key)),
        )?)
    }
}
