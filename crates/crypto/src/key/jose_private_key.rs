use crate::key::jose_key::{Key, KeyError};
use crate::key::jose_public_key::PublicKey;
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
