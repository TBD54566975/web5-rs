use crate::error::Result;
use crate::key::public_key::PublicKey;
use crypto::key::KeyType;
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

// Accidental implementation from KeyStore. Can be re-used with some finagling.
// struct CustomKeyStoreAdapter(dyn CustomKeyStore);
//
// impl CryptoKeyManager for CustomKeyStoreAdapter {
//     fn generate_private_key(&self, key_type: KeyType) -> Result<String, CryptoKeyManagerError> {
//         Ok(self.0.generate_private_key(key_type)?)
//     }
//
//     fn get_public_key(
//         &self,
//         key_alias: &str,
//     ) -> Result<Option<CryptoPublicKey>, CryptoKeyManagerError> {
//         let public_key = self.0.get(key_alias.to_string())?;
//         Ok(public_key.map(|k| k.0.clone()))
//     }
//
//     fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, CryptoKeyManagerError> {
//         Ok(self.0.sign(key_alias.to_string(), payload.to_vec())?)
//     }
//
//     fn alias(
//         &self,
//         public_key: &crypto::key::public_key::PublicKey,
//     ) -> Result<String, CryptoKeyManagerError> {
//         let public_key = crypto::key::public_key::PublicKey(public_key.clone());
//         Ok(self.0.alias(Arc::new(public_key))?)
//     }
// }
