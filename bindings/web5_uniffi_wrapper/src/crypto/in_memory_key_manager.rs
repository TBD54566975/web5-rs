use super::{
    dsa::{Signer, ToOuterSigner},
    key_manager::KeyManager,
};
use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{
    jwk::Jwk,
    key_managers::{
        in_memory_key_manager::InMemoryKeyManager as InnerInMemoryKeyManager,
        key_manager::KeyManager as InnerKeyManager,
    },
};

#[derive(Clone)]
pub struct InMemoryKeyManager(pub InnerInMemoryKeyManager);

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self(InnerInMemoryKeyManager::new())
    }

    pub fn import_private_jwk(&self, private_key: Jwk) -> Result<Jwk> {
        Ok(self.0.import_private_jwk(private_key)?)
    }

    pub fn get_as_key_manager(&self) -> Arc<dyn KeyManager> {
        Arc::new(self.clone())
    }
}

impl KeyManager for InMemoryKeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(public_jwk)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}

impl Default for InMemoryKeyManager {
    fn default() -> Self {
        Self::new()
    }
}
