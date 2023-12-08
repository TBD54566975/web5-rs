pub mod custom_key_manager;
pub mod key_store;

use crate::key_manager::key_store::KeyStore;
use crypto::key_manager::local_key_manager::LocalKeyManager;
use crypto::key_manager::KeyManager as CryptoKeyManager;
use std::sync::Arc;

pub struct KeyManager(Box<dyn CryptoKeyManager>);

impl KeyManager {
    pub fn with_key_store(key_store: Arc<KeyStore>) -> Self {
        Self(Box::new(LocalKeyManager::new(key_store)))
    }
}

// enum KeyManagerInstance {
//     CryptoKeyManager(Arc<dyn CryptoKeyManager>),
//     KeyManagerTrait(Arc<dyn CustomKeyManager>),
// }
// pub struct KeyManager(KeyManagerInstance);
//
// impl KeyManager {
//     pub fn new(key_manager: Arc<dyn CustomKeyManager>) -> Self {
//         Self(KeyManagerInstance::KeyManagerTrait(key_manager))
//     }
//
//     pub fn in_memory() -> Self {
//         let key_manager = LocalKeyManager::new(Arc::new(InMemoryKeyStore::new()));
//         Self(KeyManagerInstance::CryptoKeyManager(Arc::new(key_manager)))
//     }
// }

// impl CryptoKeyManager for KeyManager {
//     fn generate_private_key(&self, key_type: KeyType) -> Result<String, CryptoKeyManagerError> {
//         Ok(self.0.generate_private_key(key_type)?)
//     }
//
//     fn get_public_key(
//         &self,
//         key_alias: &str,
//     ) -> Result<Option<CryptoPublicKey>, CryptoKeyManagerError> {
//         let public_key = self.0.get_public_key(key_alias.to_string())?;
//         Ok(public_key.map(|k| k.0.clone()))
//     }
//
//     fn sign(&self, key_alias: &str, payload: &[u8]) -> Result<Vec<u8>, CryptoKeyManagerError> {
//         Ok(self.0.sign(key_alias.to_string(), payload.to_vec())?)
//     }
//
//     fn alias(&self, public_key: &CryptoPublicKey) -> Result<String, CryptoKeyManagerError> {
//         let public_key = PublicKey(public_key.clone());
//         Ok(self.0.alias(Arc::new(public_key))?)
//     }
// }
//
// impl<T: CryptoKeyManager> KeyManagerTrait for T {
//     fn generate_private_key(&self, key_type: KeyType) -> Result<String> {
//         let key_alias = self.generate_private_key(key_type)?;
//         Ok(key_alias)
//     }
//
//     fn get_public_key(&self, key_alias: String) -> Result<Option<Arc<PublicKey>>> {
//         let public_key = self.get_public_key(&key_alias)?;
//         Ok(public_key.map(|k| Arc::new(PublicKey(k))))
//     }
//
//     fn sign(&self, key_alias: String, payload: Vec<u8>) -> Result<Vec<u8>> {
//         let signed_payload = self.sign(&key_alias, &payload)?;
//         Ok(signed_payload)
//     }
//
//     fn alias(&self, public_key: Arc<PublicKey>) -> Result<String> {
//         let alias = self.alias(&public_key.0)?;
//         Ok(alias)
//     }
// }
