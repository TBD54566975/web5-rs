use super::key_manager::KeyManager;
use crate::{
    dsa::{OuterSigner, Signer},
    errors::Result,
};
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
        self.0
            .import_private_jwk(private_key)
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_as_key_manager(&self) -> Arc<dyn KeyManager> {
        Arc::new(self.clone())
    }
}

impl KeyManager for InMemoryKeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self
            .0
            .get_signer(public_jwk)
            .map_err(|e| Arc::new(e.into()))?;
        let outer_signer = OuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }

    fn to_inner(&self) -> Arc<dyn InnerKeyManager> {
        Arc::new(self.0.clone())
    }
}

impl Default for InMemoryKeyManager {
    fn default() -> Self {
        Self::new()
    }
}
