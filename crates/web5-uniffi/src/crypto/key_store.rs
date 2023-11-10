use crate::crypto::key::PrivateKey;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum KeyStoreError {}

#[uniffi::export]
pub trait KeyStore: Send + Sync {
    fn get(&self, key: String) -> Result<Option<PrivateKey>, KeyStoreError>;
    fn insert(&self, key: String, value: PrivateKey) -> Result<(), KeyStoreError>;
}
