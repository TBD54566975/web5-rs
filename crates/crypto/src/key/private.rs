use crate::key::{Key, KeyAlgorithm, KeyError, PublicKey};
use ssi_jwk::JWK as Jwk;
use ssi_jws::sign_bytes;

#[derive(Clone)]
pub struct PrivateKey(Jwk);

impl PrivateKey {
    pub fn generate(key_algorithm: KeyAlgorithm) -> Result<Self, KeyError> {
        let jwk = match key_algorithm {
            KeyAlgorithm::Secp256k1 => Jwk::generate_secp256k1()?,
            KeyAlgorithm::Secp256r1 => Jwk::generate_p256()?,
            KeyAlgorithm::Ed25519 => Jwk::generate_ed25519()?,
        };

        Ok(Self(jwk))
    }

    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }

    pub fn sign(&self, payload: &Vec<u8>) -> Result<Vec<u8>, KeyError> {
        let algorithm = self.0.get_algorithm().ok_or(KeyError::AlgorithmNotFound)?;

        let signed_bytes = sign_bytes(algorithm, &payload, &self.0)?;
        Ok(signed_bytes)
    }
}

impl Key for PrivateKey {
    fn jwk(&self) -> &Jwk {
        &self.0
    }
}
