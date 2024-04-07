use super::compute_thumbprint;
use crate::key::{KeyError, PublicKey};
use josekit::jwk::Jwk;
use josekit::jws::alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm};
use josekit::jws::JwsVerifier;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PublicJwk(pub Jwk);

impl PublicKey for PublicJwk {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError> {
        let verifier: Box<dyn JwsVerifier> = match self.0.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&self.0)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&self.0)?),
            _ => return Err(KeyError::AlgorithmNotFound),
        };

        verifier.verify(payload, signature).map_err(KeyError::from)
    }

    fn alias(&self) -> Result<String, KeyError> {
        if let Some(key_id) = self.0.key_id() {
            return Ok(key_id.to_string());
        }

        let key_alias = compute_thumbprint(&self.0)?;

        Ok(key_alias.to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::key::private_key::PrivateKey;
//     use josekit::jwk::alg::ec::EcCurve;

//     #[test]
//     fn test_verify() {
//         let private_key = PrivateKey(Jwk::generate_ec_key(EcCurve::Secp256k1).unwrap());
//         let signer = EcdsaJwsAlgorithm::Es256k
//             .signer_from_jwk(&private_key.jwk())
//             .unwrap();
//         let payload = b"hello world";
//         let signature = signer.sign(payload).unwrap();

//         let public_key = private_key.to_public().unwrap();
//         assert!(public_key.verify(payload, &signature).is_ok());
//     }

//     #[test]
//     fn test_verify_failure() {
//         let private_key = PrivateKey(Jwk::generate_ec_key(EcCurve::Secp256k1).unwrap());
//         let payload: &[u8] = b"hello world";
//         let signature = private_key.sign(payload).unwrap();

//         // public_key is unrelated to the private_key used to sign the payload, so it should fail
//         let public_key = PublicKey(Jwk::generate_ec_key(EcCurve::Secp256k1).unwrap());
//         let verification_warnings = public_key.verify(payload, &signature);
//         assert!(verification_warnings.is_err());
//     }
// }
