use std::sync::Arc;
use web5::apid::{
    dsa::ed25519::Ed25519Signer,
    in_memory_key_manager::InMemoryKeyManager as InnerInMemoryKeyManager, jwk::Jwk,
};

pub struct InMemoryKeyManager(InnerInMemoryKeyManager);

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            0: InnerInMemoryKeyManager::new(),
        }
    }

    pub fn generate_key_material(&self) -> Jwk {
        self.0.generate_key_material()
    }

    pub fn get_signer(&self, public_key: Jwk) -> Arc<Ed25519Signer> {
        Arc::new(self.0.get_signer(public_key))
    }

    pub fn import_key(&self, private_key: Jwk) -> Jwk {
        self.0.import_key(private_key)
    }
}
