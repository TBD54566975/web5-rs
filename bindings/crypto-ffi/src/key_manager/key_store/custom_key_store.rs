use crate::error::Result;
use crate::key::private_key::PrivateKey;
use crypto::key::private_key::PrivateKey as CryptoPrivateKey;
use crypto::key_manager::key_store::{
    KeyStore as CryptoKeyStore, KeyStoreError as CryptoKeyStoreError,
};
use std::sync::Arc;

/// A trait for foreign languages to implement their own custom key store logic.
///
/// This trait mirrors the [`crypto::key_manager::key_store::KeyStore`] trait, but with parameters
/// and return types that are compatible with `uniffi` so they can be used to generate bindings for
/// foreign languages.
pub trait CustomKeyStore: Send + Sync {
    fn get(&self, key_alias: String) -> Result<Option<Arc<PrivateKey>>>;
    fn insert(&self, key_alias: String, private_key: Arc<PrivateKey>) -> Result<()>;
}

// An adapter allows a [`CustomKeyStore`] to be used as a [`CryptoKeyStore`].
//
// Foreign languages can implement their own custom key store logic by implementing the
// [`CustomKeyStore`] trait. However, [`CustomKeyStore`] is not compatible with the Rust
// library's [`KeyStore`] trait, as the fields & return types are different.
//
// This adapter does all the necessary bridging between the two traits, allowing the
// [`CustomKeyStore`] to be used as a [`CryptoKeyStore`] within the larger Rust codebase.
pub(crate) struct CustomKeyStoreAdapter(pub(crate) Arc<dyn CustomKeyStore>);

impl CryptoKeyStore for CustomKeyStoreAdapter {
    fn get(&self, key_alias: &str) -> Result<Option<CryptoPrivateKey>, CryptoKeyStoreError> {
        let private_key = self.0.get(key_alias.to_string())?;
        Ok(private_key.map(|k| k.0.clone()))
    }

    fn insert(
        &self,
        key_alias: &str,
        private_key: CryptoPrivateKey,
    ) -> Result<(), CryptoKeyStoreError> {
        Ok(self.0.insert(
            key_alias.to_string(),
            Arc::new(PrivateKey::from(private_key)),
        )?)
    }
}
