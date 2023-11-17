use ssi_jwk::{Error as SpruceJwkError, JWK as SpruceJwk};
use ssi_jws::{sign_bytes, verify_bytes_warnable, Error as SpruceJwsError, VerificationWarnings};

pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JwkError(#[from] SpruceJwkError),
    #[error(transparent)]
    JwsError(#[from] SpruceJwsError),
    #[error("Algorithm not found")]
    AlgorithmNotFound,
}

pub trait Key {
    fn alias(&self) -> Result<String, KeyError>;
}

#[derive(Clone)]
pub struct PrivateKey {
    inner: SpruceJwk,
}

impl PrivateKey {
    pub fn new(key_algorithm: KeyAlgorithm) -> Result<Self, KeyError> {
        let inner = spruce_jwk(key_algorithm)?;
        Ok(Self { inner })
    }

    pub fn sign(&self, payload: &Vec<u8>) -> Result<Vec<u8>, KeyError> {
        let algorithm = self
            .inner
            .get_algorithm()
            .ok_or(KeyError::AlgorithmNotFound)?;

        let signed_bytes = sign_bytes(algorithm, &payload, &self.inner)?;
        Ok(signed_bytes)
    }
}

// TODO: This is duplicated between public & private key. How can we consolidate?
impl Key for PrivateKey {
    fn alias(&self) -> Result<String, KeyError> {
        Ok(self.inner.thumbprint()?)
    }
}

pub struct PublicKey {
    pub(crate) inner: SpruceJwk,
}

impl PublicKey {
    pub fn new(key_algorithm: KeyAlgorithm) -> Result<Self, KeyError> {
        let inner = spruce_jwk(key_algorithm)?.to_public();
        Ok(Self { inner })
    }

    pub fn verify(
        &self,
        payload: &Vec<u8>,
        signature: &Vec<u8>,
    ) -> Result<VerificationWarnings, KeyError> {
        let algorithm = self
            .inner
            .get_algorithm()
            .ok_or(KeyError::AlgorithmNotFound)?;

        let verification_warnings =
            verify_bytes_warnable(algorithm, &payload, &self.inner, &signature)?;

        Ok(verification_warnings)
    }
}

impl From<PrivateKey> for PublicKey {
    fn from(private_key: PrivateKey) -> Self {
        let inner = private_key.inner.to_public();
        Self { inner }
    }
}

impl Key for PublicKey {
    fn alias(&self) -> Result<String, KeyError> {
        Ok(self.inner.thumbprint()?)
    }
}

fn spruce_jwk(key_algorithm: KeyAlgorithm) -> Result<SpruceJwk, SpruceJwkError> {
    let jwk = match key_algorithm {
        KeyAlgorithm::Secp256k1 => SpruceJwk::generate_secp256k1()?,
        KeyAlgorithm::Secp256r1 => SpruceJwk::generate_p256()?,
        KeyAlgorithm::Ed25519 => SpruceJwk::generate_ed25519()?,
    };

    Ok(jwk)
}
