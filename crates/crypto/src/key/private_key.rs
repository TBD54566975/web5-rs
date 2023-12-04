use crate::key::public_key::PublicKey;
use crate::key::{Key, KeyError};
use ssi_jwk::JWK;
use ssi_jws::sign_bytes;

#[derive(Clone, PartialEq, Debug)]
pub struct PrivateKey(pub(crate) JWK);

impl PrivateKey {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }

    /// Sign a payload using the target [`PrivateKey`].
    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let algorithm = self.0.get_algorithm().ok_or(KeyError::AlgorithmNotFound)?;
        let signed_bytes = sign_bytes(algorithm, payload, &self.0)?;

        Ok(signed_bytes)
    }
}

impl Key for PrivateKey {
    fn jwk(&self) -> &JWK {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ssi_jwk::JWK;

    fn new_private_key() -> PrivateKey {
        PrivateKey(JWK::generate_secp256k1().unwrap())
    }

    #[test]
    fn test_to_public() {
        let private_key = new_private_key();
        let public_key = private_key.to_public();

        assert_eq!(
            private_key.jwk().thumbprint().unwrap(),
            public_key.jwk().thumbprint().unwrap()
        );

        assert_ne!(private_key.jwk(), public_key.jwk())
    }

    #[test]
    fn test_sign() {
        let private_key = new_private_key();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        assert_ne!(payload, &signature)
    }
}
