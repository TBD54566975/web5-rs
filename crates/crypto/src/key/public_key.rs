use crate::key::{Key, KeyError};
use josekit::{
    jwk::Jwk,
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsVerifier,
    },
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PublicKey(pub Jwk);

impl PublicKey {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    pub fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError> {
        let verifier: Box<dyn JwsVerifier> = match self.0.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&self.0)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&self.0)?),
            _ => return Err(KeyError::AlgorithmNotFound),
        };

        verifier.verify(payload, signature).map_err(KeyError::from)
    }
}

impl Key for PublicKey {
    fn jwk(&self) -> &Jwk {
        &self.0
    }
}

impl From<Jwk> for PublicKey {
    fn from(jwk: Jwk) -> Self {
        PublicKey(jwk)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::key::private_key::PrivateKey;

//     #[test]
//     fn test_verify() {
//         let private_key = PrivateKey(JWK::generate_secp256k1().unwrap());
//         let payload: &[u8] = b"hello world";
//         let signature = private_key.sign(payload).unwrap();

//         let public_key = private_key.to_public();
//         let verification_warnings = public_key.verify(payload, &signature).unwrap();
//         assert_eq!(verification_warnings.len(), 0);
//     }

//     #[test]
//     fn test_verify_failure() {
//         let private_key = PrivateKey(JWK::generate_secp256k1().unwrap());
//         let payload: &[u8] = b"hello world";
//         let signature = private_key.sign(payload).unwrap();

//         // public_key is unrelated to the private_key used to sign the payload, so it should fail
//         let public_key = PublicKey(JWK::generate_secp256k1().unwrap());
//         let verification_warnings = public_key.verify(payload, &signature);
//         assert!(verification_warnings.is_err());
//     }
// }
