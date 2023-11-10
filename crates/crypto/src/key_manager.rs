use crate::{KeyAlgorithm, PrivateKey};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum KeyManagerError {}

#[uniffi::export]
pub trait KeyManager: Send + Sync {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyManagerError>;
}
