use crate::crypto::default_key_generator::DefaultKeyGenerator;
use crate::crypto::key_generator::KeyGenerator;
use crate::crypto::key_store::KeyStore;
use std::fmt::Debug;
use std::sync::Arc;

use crate::crypto::key::{KeyAlgorithm, PrivateKey};

#[derive(uniffi::Error)]
pub struct KeyManagerError {}

#[uniffi::export]
pub trait KeyManager: Send + Sync {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyManagerError>;
}
