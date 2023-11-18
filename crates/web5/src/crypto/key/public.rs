use crate::crypto::key::{Key, KeyAlgorithm, KeyError, PrivateKey};
use ssi_jwk::JWK as Jwk;
use ssi_jws::{verify_bytes_warnable, VerificationWarnings};

pub struct PublicKey(pub(crate) Jwk);

impl PublicKey {
    pub fn generate(key_algorithm: KeyAlgorithm) -> Result<Self, KeyError> {
        let private_key = PrivateKey::generate(key_algorithm)?;
        let public_key = private_key.jwk().to_public();
        Ok(Self(public_key))
    }

    pub fn verify(
        &self,
        payload: &Vec<u8>,
        signature: &Vec<u8>,
    ) -> Result<VerificationWarnings, KeyError> {
        let algorithm = self.0.get_algorithm().ok_or(KeyError::AlgorithmNotFound)?;

        let verification_warnings =
            verify_bytes_warnable(algorithm, &payload, &self.0, &signature)?;

        Ok(verification_warnings)
    }
}

impl Key for PublicKey {
    fn jwk(&self) -> &Jwk {
        &self.0
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(private_key: PrivateKey) -> Self {
        let inner = private_key.jwk().to_public();
        Self(inner)
    }
}
