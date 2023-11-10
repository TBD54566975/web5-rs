use crate::crypto::key::{KeyAlgorithm, PrivateKey};
use std::sync::Arc;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum KeyGeneratorError {}

pub trait KeyGenerator: Send + Sync {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyGeneratorError>;
}
