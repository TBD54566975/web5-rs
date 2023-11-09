use crate::PrivateKey;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum KeyStoreError {}

#[uniffi::export]
pub trait KeyStore: Send + Sync + Debug {
    fn get(&self, key: String) -> Result<Option<Arc<PrivateKey>>, KeyStoreError>;
    fn insert(&self, key: String, value: Arc<PrivateKey>) -> Result<(), KeyStoreError>;
}
