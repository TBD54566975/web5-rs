use crate::{KeyAlgorithm, KeyStore, PrivateKey};
use ssi_jwk::JWK;
use std::sync::Arc;

// TODO: look at the kotlin AWS implementation, might need to tweak
// this file to allow for the same constructs here...

#[derive(uniffi::Object)]
pub struct KeyManager {
    key_store: Arc<dyn KeyStore>,
}

#[uniffi::export]
impl KeyManager {
    #[uniffi::constructor]
    pub fn new(key_store: Arc<dyn KeyStore>) -> Arc<Self> {
        Self { key_store }.into()
    }
}

impl KeyManager {
    pub(crate) fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> PrivateKey {
        let jwk: JWK;
        match key_algorithm {
            KeyAlgorithm::Secp256k1 => {
                jwk = JWK::generate_secp256k1().unwrap();
            }
            KeyAlgorithm::Secp256r1 => {
                jwk = JWK::generate_p256().unwrap();
            }
            KeyAlgorithm::Ed25519 => {
                jwk = JWK::generate_ed25519().unwrap();
            }
        }

        let private_key = PrivateKey(jwk);

        // let alias = Self::get_alias(&jwk);
        // // TODO: investigate how a foreign trait implementation can throw
        // let _ = self.key_store.insert(alias.clone(), private_key.clone());

        private_key
    }
}
