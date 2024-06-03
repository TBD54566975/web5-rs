use crate::crypto::{ed25519::Ed25519, secp256k1::Secp256k1, CryptoError, CurveOperations};
use crate::jwk::{Jwk, JwkError};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum KeyError {
    #[error("key generation failed")]
    KeyGenerationFailed,
    #[error("failed to serialize")]
    SerializationFailed,
    #[error("curve not found")]
    CurveNotFound,
    #[error("algorithm not found")]
    AlgorithmNotFound,
    #[error(transparent)]
    JwkError(#[from] JwkError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub trait Key: Send + Sync {
    fn alias(&self) -> Result<String, KeyError>;
    fn jwk(&self) -> Result<Jwk, KeyError>;
}

pub trait PublicKey: Key + Send + Sync {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError>;
}

pub trait PrivateKey: Key + Send + Sync {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError>;

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError>;
}

impl Key for Jwk {
    fn alias(&self) -> Result<String, KeyError> {
        let thumbprint = self.compute_thumbprint()?;
        Ok(thumbprint)
    }

    fn jwk(&self) -> Result<Jwk, KeyError> {
        Ok(self.clone())
    }
}

impl PublicKey for Jwk {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError> {
        match self.crv.as_str() {
            "Ed25519" => Ed25519::verify(self, payload, signature),
            "secp256k1" => Secp256k1::verify(self, payload, signature),
            _ => return Err(KeyError::CurveNotFound),
        }?;

        Ok(())
    }
}

impl PrivateKey for Jwk {
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError> {
        let public_key = self.to_public_jwk();
        Ok(Arc::new(public_key))
    }

    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let signature = match self.crv.as_str() {
            "Ed25519" => Ed25519::sign(self, payload),
            "secp256k1" => Secp256k1::sign(self, payload),
            _ => return Err(KeyError::CurveNotFound),
        }?;
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::{ed25519::Ed25519, CurveOperations};

    use super::*;

    #[test]
    fn test_verify() {
        let private_key = Ed25519::generate().unwrap();
        let payload = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }

    #[test]
    fn test_verify_failure() {
        let private_key = Ed25519::generate().unwrap();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        // public_key is unrelated to the private_key used to sign the payload, so it should fail
        let private_key_2 = Ed25519::generate().unwrap();
        let public_key = private_key_2.to_public().unwrap();
        let verification_warnings = public_key.verify(payload, &signature);
        assert!(verification_warnings.is_err());
    }

    #[test]
    fn test_to_public() {
        let private_key = Ed25519::generate().unwrap();
        let public_key = private_key.to_public().unwrap().jwk().unwrap();

        assert_eq!(private_key.x, public_key.x);
        assert_eq!(private_key.y, public_key.y);

        assert!(private_key.d.is_some());
        assert!(public_key.d.is_none());
    }

    #[test]
    fn test_sign() {
        let private_key = Ed25519::generate().unwrap();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }
}
