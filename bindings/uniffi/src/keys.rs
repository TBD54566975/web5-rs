use std::sync::Arc;

use crate::inner::{
    dsa::Ed25519Signer,
    keys::{InMemoryKeyManager as InnerInMemoryKeyManager, Jwk},
};

pub struct InMemoryKeyManager(InnerInMemoryKeyManager);

impl InMemoryKeyManager {
    // 🚧 not in APID
    pub fn new() -> Self {
        Self(InnerInMemoryKeyManager {})
    }

    pub fn generate_key_material(&self) -> Jwk {
        self.0.generate_key_material()
    }

    // 🚧 Arc not in inner
    pub fn get_signer(&self, public_key: Jwk) -> Arc<Ed25519Signer> {
        Arc::new(self.0.get_signer(public_key))
    }

    pub fn import_key(&self, private_key: Jwk) -> Jwk {
        self.0.import_key(private_key)
    }
}
