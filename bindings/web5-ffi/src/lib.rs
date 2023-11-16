uniffi::include_scaffolding!("web5_ffi");

use std::sync::Arc;

pub use web5::crypto::key::KeyAlgorithm;
pub use web5::crypto::key::PrivateKey;
pub use web5::crypto::key_manager::local_key_manager::LocalKeyManager;
pub use web5::crypto::key_manager::{KeyManager, KeyManagerError};

// Super hacky way to get pure Rust trait exposed as a foreign implementation trait.
// I have tried multiple other ways, with no success. I would love if someone could
// show me a better way to do this.
//
// What the below does:
// * Rename the pure rust trait to RustKeyStore
// * Define a uniffi-compatible KeyStore trait, with supported params & return types
// * Create a wrapper struct, which will execute the foreign language implementation when
//   the pure rust trait is called.
//
// This means we're creating some additional memory, because it's cloning the values that
// come in from the foreign language implementation.
//
// :shrug: Tradeoffs :shrug:

pub use web5::crypto::key_store::{KeyStore as RustKeyStore, KeyStoreError};

pub trait KeyStore: Send + Sync {
    fn get_private_key(&self, key_alias: String) -> Result<Option<Arc<PrivateKey>>, KeyStoreError>;
    fn insert_private_key(
        &self,
        key_alias: String,
        private_key: Arc<PrivateKey>,
    ) -> Result<(), KeyStoreError>;
}

pub struct KeyStoreWrapper(Arc<dyn KeyStore>);

impl RustKeyStore for KeyStoreWrapper {
    fn get(&self, key_alias: &str) -> Result<Option<PrivateKey>, KeyStoreError> {
        let private_key = self.0.get_private_key(key_alias.to_string())?;

        if let Some(private_key) = private_key {
            Ok(Some((*private_key).clone()))
        } else {
            Ok(None)
        }
    }

    fn insert(&self, key_alias: &str, private_key: PrivateKey) -> Result<(), KeyStoreError> {
        let key_alias = key_alias.to_string();
        let private_key = Arc::new(private_key);
        self.0.insert_private_key(key_alias, private_key)
    }
}

// Expose a constructor for LocalKeyManager

pub fn local_key_manager(key_store: Arc<dyn KeyStore>) -> Arc<LocalKeyManager> {
    let wrapper = KeyStoreWrapper(key_store);
    let local_key_manager = LocalKeyManager::new(Arc::new(wrapper));
    Arc::new(local_key_manager)
}
