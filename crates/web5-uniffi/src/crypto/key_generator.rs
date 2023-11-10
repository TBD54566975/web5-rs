use crate::{KeyAlgorithm, PrivateKey};
use ssi_jwk::JWK;

pub fn generate_private_key(key_algorithm: KeyAlgorithm) -> PrivateKey {
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

    PrivateKey(jwk)
}
