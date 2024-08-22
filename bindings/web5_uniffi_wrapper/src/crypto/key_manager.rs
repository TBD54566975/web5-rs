use super::dsa::{Signer, ToInnerSigner, ToOuterSigner};
use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{jwk::Jwk, key_managers::KeyManager as InnerKeyManager};

pub trait KeyManager: Send + Sync {
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk>;
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
}

pub struct ToOuterKeyManager(pub Arc<dyn InnerKeyManager>);

impl KeyManager for ToOuterKeyManager {
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk> {
        Ok(self.0.import_private_jwk(private_jwk)?)
    }

    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        let signer = self.0.get_signer(public_jwk)?;
        let outer_signer = ToOuterSigner(signer);
        Ok(Arc::new(outer_signer))
    }
}

pub struct ToInnerKeyManager(pub Arc<dyn KeyManager>);

impl InnerKeyManager for ToInnerKeyManager {
    fn import_private_jwk(&self, private_jwk: Jwk) -> web5::errors::Result<Jwk> {
        Ok(self.0.import_private_jwk(private_jwk)?)
    }

    fn get_signer(
        &self,
        public_jwk: Jwk,
    ) -> web5::errors::Result<Arc<dyn web5::crypto::dsa::Signer>> {
        let outer_signer = self.0.get_signer(public_jwk)?;
        let inner_signer = Arc::new(ToInnerSigner(outer_signer));
        Ok(inner_signer)
    }
}
