use crate::crypto::key::{Key, KeyAlgorithm, PrivateKey};
use crate::crypto::key_manager::{KeyManager, KeyManagerError};
use ssi_jwk::JWK;
use std::sync::Arc;

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum LocalKeyStoreError {
    #[error("An unknown error occurred")]
    Unknown,
}

#[uniffi::export]
pub trait LocalKeyStore: Send + Sync {
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, LocalKeyStoreError>;
    fn insert(&self, key: String, value: &PrivateKey) -> Result<(), LocalKeyStoreError>;
}

#[derive(uniffi::Object)]
pub struct LocalKeyManager {
    key_store: Arc<dyn LocalKeyStore>,
}

#[uniffi::export]
impl LocalKeyManager {
    #[uniffi::constructor]
    fn new(key_store: Arc<dyn LocalKeyStore>) -> Arc<Self> {
        Self { key_store }.into()
    }
}

impl KeyManager for LocalKeyManager {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyManagerError> {
        let jwk: JWK;

        match key_algorithm {
            KeyAlgorithm::Secp256k1 => {
                jwk = JWK::generate_secp256k1().unwrap();
            }
            KeyAlgorithm::Secp256r1 => {
                jwk = JWK::generate_p256().unwrap();
            }
            KeyAlgorithm::Ed25519 => {
                jwk = JWK::generate_ed25519().unwrap();
            }
        }

        let private_key = PrivateKey(jwk);
        // TODO: handle error case once I understand a bit more
        let _ = self.key_store.insert(private_key.alias(), &private_key);

        Ok(private_key.into())
    }
}
