use super::compute_thumbprint;
use crate::key::josekit_jwk::public_jwk::PublicJwk;
use crate::key::{KeyError, KeyType, PrivateKey, PublicKey};
use josekit::jwk::alg::ec::EcCurve;
use josekit::jwk::alg::ed::EdCurve;
use josekit::jwk::Jwk;
use josekit::jws::alg::ecdsa::EcdsaJwsAlgorithm;
use josekit::jws::alg::eddsa::EddsaJwsAlgorithm;
use josekit::jws::JwsSigner;

#[derive(Clone, PartialEq, Debug)]
pub struct PrivateJwk(pub(crate) Jwk);

impl PrivateKey for PrivateJwk {
    fn generate(key_type: crate::key::KeyType) -> Result<Self, KeyError> {
        let mut jwk = match key_type {
            KeyType::Secp256k1 => Jwk::generate_ec_key(EcCurve::Secp256k1),
            KeyType::Ed25519 => Jwk::generate_ed_key(EdCurve::Ed25519),
        }?;

        let key_alias = compute_thumbprint(&jwk)?;
        jwk.set_key_id(&key_alias);

        Ok(Self(jwk))
    }

    /// Derive a [`PublicJwk`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<Box<dyn PublicKey>, KeyError> {
        let mut public_key = self.0.to_public_key()?;

        let key_alias = compute_thumbprint(&public_key)?;
        public_key.set_key_id(&key_alias);

        Ok(Box::from(PublicJwk(public_key)))
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use josekit::jwk::alg::ec::EcCurve;

//     fn new_private_key() -> PrivateKey {
//         PrivateKey(Jwk::generate_ec_key(EcCurve::Secp256k1).unwrap())
//     }

//     #[test]
//     fn test_to_public() {
//         let private_key = new_private_key();
//         let public_key = private_key.to_public().unwrap();

//         assert_eq!(
//             private_key.jwk().parameter("x"),
//             public_key.jwk().parameter("x")
//         );
//         assert_eq!(
//             private_key.jwk().parameter("y"),
//             public_key.jwk().parameter("y")
//         );

//         assert!(private_key.jwk().parameter("d").is_some());
//         assert!(public_key.jwk().parameter("d").is_none());
//     }

//     #[test]
//     fn test_sign() {
//         let private_key = new_private_key();
//         let payload: &[u8] = b"hello world";
//         let signature = private_key.sign(payload).unwrap();

//         let public_key = private_key.to_public().unwrap();
//         let verifier = EcdsaJwsAlgorithm::Es256k
//             .verifier_from_jwk(&public_key.jwk())
//             .unwrap();
//         assert!(verifier.verify(payload, &signature).is_ok());
//     }
// }
