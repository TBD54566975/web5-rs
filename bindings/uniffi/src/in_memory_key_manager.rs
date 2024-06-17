use crate::{
    dsa::{ed25519::Ed25519Signer, Signer},
    errors::Result,
};
use std::sync::Arc;
use web5::apid::{in_memory_key_manager::InMemoryKeyManager as InnerInMemoryKeyManager, jwk::Jwk};

pub struct InMemoryKeyManager(pub InnerInMemoryKeyManager);

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self(InnerInMemoryKeyManager::new())
    }

    pub fn generate_key_material(&self) -> Result<Jwk> {
        self.0
            .generate_key_material()
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_signer(&self, public_key: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self
            .0
            .get_signer(public_key)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Arc::new(Ed25519Signer(signer)))
    }

    pub fn import_key(&self, private_key: Jwk) -> Result<Jwk> {
        self.0
            .import_key(private_key)
            .map_err(|e| Arc::new(e.into()))
    }
}
