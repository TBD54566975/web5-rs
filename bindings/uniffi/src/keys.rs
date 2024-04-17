use crate::Jwk;
use crypto::Curve;
use jwk::Jwk as InternalJwk;
use keys::{
    key::PrivateKey,
    key_manager::{
        local_key_manager::LocalKeyManager as InternalLocalKeyManager, KeyManager, KeyManagerError,
    },
};
use std::sync::Arc;

pub struct LocalJwkManager(InternalLocalKeyManager);

impl LocalJwkManager {
    pub fn new() -> Self {
        Self(InternalLocalKeyManager::new_in_memory())
    }

    pub fn generate_private_key(
        &self,
        curve: Curve,
        key_alias: Option<String>,
    ) -> Result<String, KeyManagerError> {
        self.0
            .generate_private_key(curve, key_alias.as_ref().map(|x| x.as_str()))
    }

    pub fn get_public_key(&self, key_alias: String) -> Result<Arc<Jwk>, KeyManagerError> {
        let public_key = self.0.get_public_key(&key_alias)?;
        let internal_public_jwk = public_key.jwk()?;
        let public_jwk = Jwk(internal_public_jwk);
        Ok(Arc::new(public_jwk))
    }

    pub fn sign(&self, key_alias: String, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        self.0.sign(&key_alias, payload)
    }

    // todo not implemented
    // pub fn get_signer(&self, key_alias: String) -> Result<Signer, KeyManagerError> {
    //     unimplemented!()
    // }

    pub fn export_private_keys(&self) -> Result<Vec<Arc<Jwk>>, KeyManagerError> {
        let private_keys = self.0.export_private_keys()?;
        let mut private_jwks: Vec<Arc<Jwk>> = Vec::new();
        for private_key in private_keys {
            let private_internal_jwk = private_key.jwk()?;
            let private_jwk = Jwk::from(private_internal_jwk);
            private_jwks.push(Arc::new(private_jwk));
        }
        Ok(private_jwks)
    }

    pub fn import_private_keys(&self, private_keys: Vec<Arc<Jwk>>) -> Result<(), KeyManagerError> {
        let mut private_internal_jwks: Vec<Arc<dyn PrivateKey>> = Vec::new();
        for private_key in private_keys {
            let private_internal_jwk = InternalJwk::from(private_key.as_ref());
            let private_internal_jwk_dyn: Arc<dyn PrivateKey> = Arc::new(private_internal_jwk);
            private_internal_jwks.push(private_internal_jwk_dyn);
        }
        self.0.import_private_keys(private_internal_jwks)?;
        Ok(())
    }
}
