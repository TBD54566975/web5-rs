use crate::key::jose_key::{Key, KeyError};
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
