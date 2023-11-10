use crate::crypto::key_generator::KeyGenerator;
use crate::crypto::key_store::KeyStore;
use std::fmt::Debug;
use std::sync::Arc;
use crate::crypto::default_key_generator::DefaultKeyGenerator;

#[uniffi::export]
pub struct KeyManager {
    key_generator: Arc<dyn KeyGenerator>,
    key_store: Arc<dyn KeyStore>,
}

#[uniffi::export]
impl KeyManager {
    #[uniffi::constructor]
    pub fn new(
        key_generator: Option<Arc<dyn KeyGenerator>>,
        key_store: Option<Arc<dyn KeyStore>>,
    ) -> Arc<Self> {
        let a = key_generator.unwrap_or(DefaultKeyGenerator::new().into())

        Self {
            key_generator,
            key_store,
        }
        .into()
    }
}
