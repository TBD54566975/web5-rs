use std::sync::Arc;

use crypto::Curve;
use jwk::Jwk;
use keys::{
    key::PrivateKey,
    key_manager::{
        local_key_manager::LocalKeyManager as InternalLocalKeyManager, KeyManager, KeyManagerError,
    },
};

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

    pub fn get_public_key(&self, key_alias: String) -> Result<Jwk, KeyManagerError> {
        let public_key = self.0.get_public_key(&key_alias)?;
        let public_jwk = public_key.jwk()?;
        Ok(public_jwk)
    }

    pub fn sign(&self, key_alias: String, payload: &[u8]) -> Result<Vec<u8>, KeyManagerError> {
        self.0.sign(&key_alias, payload)
    }

    pub fn export_private_keys(&self) -> Result<Vec<Jwk>, KeyManagerError> {
        let private_keys = self.0.export_private_keys()?;
        let mut private_jwks: Vec<Jwk> = Vec::new();
        for private_key in private_keys {
            let private_jwk = private_key.jwk()?;
            private_jwks.push(private_jwk);
        }
        Ok(private_jwks)
    }

    pub fn import_private_keys(&self, private_keys: Vec<Jwk>) -> Result<(), KeyManagerError> {
        let mut private_jwks: Vec<Arc<dyn PrivateKey>> = Vec::new();
        for private_key in private_keys {
            private_jwks.push(Arc::new(private_key));
        }
        self.0.import_private_keys(private_jwks)?;
        Ok(())
    }
}
