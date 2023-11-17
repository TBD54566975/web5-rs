use crate::crypto::key::{Key, KeyAlgorithm, KeyError};
use ssi_jwk::{Error as SpruceJwkError, JWK as SpruceJwk};
use ssi_jws::sign_bytes;

#[derive(Clone)]
pub struct PrivateKey {
    pub(crate) inner: SpruceJwk,
}

impl PrivateKey {
    pub fn new(key_algorithm: KeyAlgorithm) -> Result<Self, KeyError> {
        let inner = spruce_jwk(key_algorithm)?;
        Ok(Self { inner })
    }

    pub fn sign(&self, payload: &Vec<u8>) -> Result<Vec<u8>, KeyError> {
        let algorithm = self
            .inner
            .get_algorithm()
            .ok_or(KeyError::AlgorithmNotFound)?;

        let signed_bytes = sign_bytes(algorithm, &payload, &self.inner)?;
        Ok(signed_bytes)
    }
}

// TODO: This is duplicated between public & private key. How can we consolidate?
impl Key for PrivateKey {
    fn alias(&self) -> Result<String, KeyError> {
        Ok(self.inner.thumbprint()?)
    }
}

fn spruce_jwk(key_algorithm: KeyAlgorithm) -> Result<SpruceJwk, SpruceJwkError> {
    let jwk = match key_algorithm {
        KeyAlgorithm::Secp256k1 => SpruceJwk::generate_secp256k1()?,
        KeyAlgorithm::Secp256r1 => SpruceJwk::generate_p256()?,
        KeyAlgorithm::Ed25519 => SpruceJwk::generate_ed25519()?,
    };

    Ok(jwk)
}
