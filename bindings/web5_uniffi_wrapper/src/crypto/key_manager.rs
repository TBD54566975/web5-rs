use super::dsa::{OuterSigner, Signer};
use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{jwk::Jwk, key_managers::key_manager::KeyManager as InnerKeyManager};

pub trait KeyManager: Send + Sync {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
    fn to_inner(&self) -> Arc<dyn InnerKeyManager>;
}

pub struct OuterKeyManager(pub Arc<dyn InnerKeyManager>);

impl KeyManager for OuterKeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self
            .0
            .get_signer(public_jwk)
            .map_err(|e| Arc::new(e.into()))?;
        let outer_signer = OuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }

    fn to_inner(&self) -> Arc<dyn InnerKeyManager> {
        self.0.clone()
    }
}
