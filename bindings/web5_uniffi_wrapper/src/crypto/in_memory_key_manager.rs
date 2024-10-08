use super::{
    dsa::{Signer, ToOuterSigner},
    key_exporter::KeyExporter,
    key_manager::KeyManager,
};
use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{
    jwk::Jwk,
    key_managers::{
        in_memory_key_manager::InMemoryKeyManager as InnerInMemoryKeyManager,
        KeyExporter as InnerKeyExporter, KeyManager as InnerKeyManager,
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
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk> {
        Ok(self.0.import_private_jwk(private_jwk)?)
    }

    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(public_jwk)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}

impl KeyExporter for InMemoryKeyManager {
    fn export_private_jwks(&self) -> Result<Vec<Jwk>> {
        let private_jwks = self.0.export_private_jwks()?;
        Ok(private_jwks)
    }
}

impl Default for InMemoryKeyManager {
    fn default() -> Self {
        Self::new()
    }
}
