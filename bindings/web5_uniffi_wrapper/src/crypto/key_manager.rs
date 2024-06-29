use super::dsa::{ToOuterSigner, Signer, ToInnerSigner};
use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{jwk::Jwk, key_managers::key_manager::KeyManager as InnerKeyManager};

pub trait KeyManager: Send + Sync {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
}

pub struct ToOuterKeyManager(pub Arc<dyn InnerKeyManager>);

impl KeyManager for ToOuterKeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(public_jwk)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}

pub struct ToInnerKeyManager(pub Arc<dyn KeyManager>);

impl InnerKeyManager for ToInnerKeyManager {
    fn get_signer(
        &self,
        public_jwk: Jwk,
    ) -> web5::crypto::key_managers::Result<Arc<dyn web5::crypto::dsa::Signer>> {
        let outer_signer = self.0.get_signer(public_jwk).unwrap(); // 🚧 unwrap, need a .into() I think
        let inner_signer = Arc::new(ToInnerSigner(outer_signer));
        Ok(inner_signer)
    }
}
