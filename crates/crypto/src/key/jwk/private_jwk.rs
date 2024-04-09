use super::compute_thumbprint;
use super::public_jwk::PublicJwk;
use crate::key::{Key, KeyError, PrivateKey, PublicKey};
use josekit::jwk::Jwk;
use josekit::jws::alg::ecdsa::EcdsaJwsAlgorithm;
use josekit::jws::alg::eddsa::EddsaJwsAlgorithm;
use josekit::jws::JwsSigner;
use std::sync::Arc;

#[derive(Clone, PartialEq, Debug)]
pub struct PrivateJwk(pub(crate) Jwk);

impl PrivateKey for PrivateJwk {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError> {
        let mut public_key = self.0.to_public_key()?;

        let key_alias = compute_thumbprint(&public_key)?;
        public_key.set_key_id(&key_alias);
        public_key.set_algorithm(match self.0.curve() {
            Some("secp256k1") => EcdsaJwsAlgorithm::Es256k.to_string(),
            Some("Ed25519") => EddsaJwsAlgorithm::Eddsa.to_string(),
            _ => return Err(KeyError::AlgorithmNotFound),
        });

        Ok(Arc::new(PublicJwk(public_key)))
    }

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let signer: Box<dyn JwsSigner> = match self.0.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.signer_from_jwk(&self.0)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.signer_from_jwk(&self.0)?),
            _ => return Err(KeyError::AlgorithmNotFound),
        };

        signer.sign(payload).map_err(KeyError::from)
    }
}

impl Key for PrivateJwk {
    fn jwk(&self) -> &Jwk {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::{jwk::generate_private_jwk, KeyType};

    #[test]
    fn test_to_public() {
        let private_key = generate_private_jwk(KeyType::Secp256k1).unwrap();
        let public_key = private_key.to_public().unwrap();

        assert_eq!(
            private_key.jwk().parameter("x"),
            public_key.jwk().parameter("x")
        );
        assert_eq!(
            private_key.jwk().parameter("y"),
            public_key.jwk().parameter("y")
        );

        assert!(private_key.jwk().parameter("d").is_some());
        assert!(public_key.jwk().parameter("d").is_none());
    }

    #[test]
    fn test_sign() {
        let private_key = generate_private_jwk(KeyType::Secp256k1).unwrap();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }
}
