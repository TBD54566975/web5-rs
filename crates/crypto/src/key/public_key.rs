use crate::key::{Key, KeyError};
use serde::{Deserialize, Serialize};
use ssi_jwk::JWK;
use ssi_jws::{verify_bytes_warnable, VerificationWarnings};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PublicKey(pub(crate) JWK);

impl PublicKey {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    pub fn verify(
        &self,
        payload: &[u8],
        signature: &[u8],
    ) -> Result<VerificationWarnings, KeyError> {
        let algorithm = self.0.get_algorithm().ok_or(KeyError::AlgorithmNotFound)?;

        let verification_warnings = verify_bytes_warnable(algorithm, payload, &self.0, signature)?;

        Ok(verification_warnings)
    }
}

impl Key for PublicKey {
    fn jwk(&self) -> &JWK {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::private_key::PrivateKey;

    #[test]
    fn test_verify() {
        let private_key = PrivateKey(JWK::generate_secp256k1().unwrap());
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public();
        let verification_warnings = public_key.verify(payload, &signature).unwrap();
        assert_eq!(verification_warnings.len(), 0);
    }

    #[test]
    fn test_verify_failure() {
        let private_key = PrivateKey(JWK::generate_secp256k1().unwrap());
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        // public_key is unrelated to the private_key used to sign the payload, so it should fail
        let public_key = PublicKey(JWK::generate_secp256k1().unwrap());
        let verification_warnings = public_key.verify(payload, &signature);
        assert!(verification_warnings.is_err());
    }
}
