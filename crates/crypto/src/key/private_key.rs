use crate::key::public_key::PublicKey;
use crate::key::{Key, KeyError};
use josekit::jwk::Jwk;
use josekit::jws::alg::ecdsa::EcdsaJwsAlgorithm;
use josekit::jws::alg::eddsa::EddsaJwsAlgorithm;
use josekit::jws::JwsSigner;

#[derive(Clone, PartialEq, Debug)]
pub struct PrivateKey(pub(crate) Jwk);

impl PrivateKey {
    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    pub fn to_public(&self) -> Result<PublicKey, KeyError> {
        let public_key = self.0.to_public_key()?;

        Ok(PublicKey(public_key))
    }

    /// Sign a payload using the target [`PrivateKey`].
    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let signer: Box<dyn JwsSigner> = match self.0.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.signer_from_jwk(&self.0)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.signer_from_jwk(&self.0)?),
            _ => return Err(KeyError::AlgorithmNotFound),
        };

        signer.sign(payload).map_err(KeyError::from)
    }
}

impl Key for PrivateKey {
    fn jwk(&self) -> &Jwk {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use josekit::jwk::alg::ec::EcCurve;

    fn new_private_key() -> PrivateKey {
        PrivateKey(Jwk::generate_ec_key(EcCurve::Secp256k1).unwrap())
    }

    #[test]
    fn test_to_public() {
        let private_key = new_private_key();
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
        let private_key = new_private_key();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        let verifier = EcdsaJwsAlgorithm::Es256k
            .verifier_from_jwk(&public_key.jwk())
            .unwrap();
        assert!(verifier.verify(payload, &signature).is_ok());
    }
}
