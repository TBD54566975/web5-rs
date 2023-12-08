use crate::error::Result;
use crate::key::public_key::PublicKey;
use crypto::key::public_key::PublicKey as CryptoPublicKey;
use crypto::key::KeyType;
use crypto::key_manager::{
    KeyManager as CryptoKeyManager, KeyManagerError as CryptoKeyManagerError,
};
use std::sync::Arc;

/// A trait for foreign languages to implement their own custom key manager logic.
///
/// This trait mirrors the [`crypto::key_manager::KeyManager`] trait, but with parameters and return
/// types that are compatible with `uniffi` so they can be used to generate bindings for foreign
/// languages.
pub trait CustomKeyManager: Send + Sync {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String>;
    fn get_public_key(&self, key_alias: String) -> Result<Option<Arc<PublicKey>>>;
    fn sign(&self, key_alias: String, payload: Vec<u8>) -> Result<Vec<u8>>;
    fn alias(&self, public_key: Arc<PublicKey>) -> Result<String>;
}

// An adapter which allows a [`CustomKeyManager`] to be used as a [`CryptoKeyManager`].
//
// Foreign languages can implement their own custom key manager logic by implementing the
// [`CustomKeyManager`] trait. However, [`CustomKeyManager`] is not compatible with the Rust
// library's [`KeyManager`] trait, as the fields & return types are different.
//
// This adapter does all the necessary bridging between the two traits, allowing the
// [`CustomKeyManager`] to be used as a [`CryptoKeyManager`] within the larger Rust codebase.
pub(crate) struct CustomKeyManagerAdapter(pub(crate) Arc<dyn CustomKeyManager>);

impl CryptoKeyManager for CustomKeyManagerAdapter {
    fn generate_private_key(&self, key_type: KeyType) -> Result<String, CryptoKeyManagerError> {
        Ok(self.0.generate_private_key(key_type)?)
    }

    fn get_public_key(
        &self,
        key_alias: &str,
    ) -> Result<Option<CryptoPublicKey>, CryptoKeyManagerError> {
        Ok(self
            .0
            .get_public_key(key_alias.to_string())?
            .map(|pk| pk.0.clone()))
    }

    fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, CryptoKeyManagerError> {
        Ok(self.0.sign(key_alias.to_string(), payload.to_vec())?)
    }
    fn alias(&self, public_key: &CryptoPublicKey) -> Result<String, CryptoKeyManagerError> {
        Ok(self
            .0
            .alias(Arc::new(PublicKey::from(public_key.clone())))?)
    }
}
