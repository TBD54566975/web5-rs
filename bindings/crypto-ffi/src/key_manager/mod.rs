pub mod custom_key_manager;
pub mod key_store;

use crate::key_manager::custom_key_manager::CustomKeyManager;
use crate::key_manager::key_store::KeyStore;
use crypto::key::public_key::PublicKey as CryptoPublicKey;
use crypto::key::KeyType;
use crypto::key_manager::local_key_manager::LocalKeyManager;
use crypto::key_manager::{
    KeyManager as CryptoKeyManager, KeyManagerError as CryptoKeyManagerError,
};
use custom_key_manager::CustomKeyManagerAdapter;
use std::sync::Arc;

/// Concrete `KeyManager` struct, exposed to foreign languages.
///
/// This struct is a new type around the [`crypto::key_manager::KeyManager`] trait, which is not
/// exposed to foreign languages because it is incompatible with FFI. However, the
/// [`crypto::key_manager::KeyManager`] trait is used as a parameter for various other areas
/// within the `web5-rs` codebase (e.g. creating a DID). This struct allows foreign languages to
/// instantiate one of those trait objects, and then use it in those various other areas.
pub struct KeyManager(Box<dyn CryptoKeyManager>);

impl KeyManager {
    pub fn new(custom: Arc<dyn CustomKeyManager>) -> Self {
        Self(Box::new(CustomKeyManagerAdapter(custom)))
    }

    pub fn new_with_key_store(key_store: Arc<KeyStore>) -> Self {
        Self(Box::new(LocalKeyManager::new(key_store)))
    }
}

impl CryptoKeyManager for KeyManager {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, CryptoKeyManagerError> {
        Ok(self.0.generate_private_key(key_type)?)
    }

    fn get_public_key(
        &self,
        key_alias: &str,
    ) -> Result<Option<CryptoPublicKey>, CryptoKeyManagerError> {
        Ok(self.0.get_public_key(key_alias)?)
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, CryptoKeyManagerError> {
        Ok(self.0.sign(key_alias, payload)?)
    }

    fn alias(&self, public_key: &CryptoPublicKey) -> Result<String, CryptoKeyManagerError> {
        Ok(self.0.alias(public_key)?)
    }
}
