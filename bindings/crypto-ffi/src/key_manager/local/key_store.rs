use crate::error::Result;
use crate::key::PrivateKey;
use crypto::key::PrivateKey as RustPrivateKey;
use crypto::key_manager::local::key_store::{
    KeyStore as RustKeyStore, KeyStoreError as RustKeyStoreError,
};
use std::sync::Arc;

pub trait KeyStore: Send + Sync {
    fn get_private_key(&self, key_alias: String) -> Result<Option<Arc<PrivateKey>>>;
    // TODO: can we get rid of Arc by using [ByRef]?
    fn insert_private_key(&self, key_alias: String, private_key: Arc<PrivateKey>) -> Result<()>;
}

pub struct KeyStoreWrapper(pub(crate) Arc<dyn KeyStore>);

impl RustKeyStore for KeyStoreWrapper {
    fn get(&self, key_alias: &str) -> Result<Option<RustPrivateKey>, RustKeyStoreError> {
        let private_key = self.0.get_private_key(key_alias.to_string())?;

        if let Some(private_key) = private_key {
            Ok(Some(private_key.inner.clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(
        &self,
        key_alias: &str,
        private_key: RustPrivateKey,
    ) -> Result<(), RustKeyStoreError> {
        let key_alias = key_alias.to_string();
        let private_key = Arc::new(private_key.into());

        self.0.insert_private_key(key_alias, private_key)?;

        Ok(())
    }
}
