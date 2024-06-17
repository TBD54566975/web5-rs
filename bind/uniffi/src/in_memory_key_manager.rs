use crate::{
    dsa::{ed25519::RcbEd25519Signer, RcbSigner},
    errors::RcbResult,
};
use std::sync::Arc;
use web5::apid::{in_memory_key_manager::InMemoryKeyManager, jwk::Jwk};

pub struct RcbInMemoryKeyManager(pub InMemoryKeyManager);

impl RcbInMemoryKeyManager {
    pub fn new() -> Self {
        Self(InMemoryKeyManager::new())
    }

    pub fn generate_key_material(&self) -> RcbResult<Jwk> {
        self.0
            .generate_key_material()
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_signer(&self, public_key: Jwk) -> RcbResult<Arc<dyn RcbSigner>> {
        let signer = self
            .0
            .get_signer(public_key)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Arc::new(RcbEd25519Signer(signer)))
    }

    pub fn import_key(&self, private_key: Jwk) -> RcbResult<Jwk> {
        self.0
            .import_key(private_key)
            .map_err(|e| Arc::new(e.into()))
    }
}
