use crate::crypto::key::{KeyAlgorithm, PrivateKey};
use crate::crypto::key_generator::{KeyGenerator, KeyGeneratorError};
use ssi_jwk::JWK;
use std::sync::Arc;

pub struct DefaultKeyGenerator {}

impl KeyGenerator for DefaultKeyGenerator {
    fn generate_private_key(
        &self,
        key_algorithm: KeyAlgorithm,
    ) -> Result<Arc<PrivateKey>, KeyGeneratorError> {
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

        PrivateKey(jwk).into()
    }
}
