use crate::crypto::key::private_key::PrivateKey;
use crate::crypto::key::{Key, KeyError};
use ssi_jwk::JWK as SpruceJwk;
use ssi_jws::{sign_bytes, verify_bytes_warnable, Error as SpruceJwsError, VerificationWarnings};

pub struct PublicKey {
    pub(crate) inner: SpruceJwk,
}

impl PublicKey {
    pub fn verify(
        &self,
        payload: &Vec<u8>,
        signature: &Vec<u8>,
    ) -> Result<VerificationWarnings, KeyError> {
        let algorithm = self
            .inner
            .get_algorithm()
            .ok_or(KeyError::AlgorithmNotFound)?;

        let verification_warnings =
            verify_bytes_warnable(algorithm, &payload, &self.inner, &signature)?;

        Ok(verification_warnings)
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(private_key: PrivateKey) -> Self {
        let inner = private_key.inner.to_public();
        Self { inner }
    }
}

impl Key for PublicKey {
    fn alias(&self) -> Result<String, KeyError> {
        Ok(self.inner.thumbprint()?)
    }
}
