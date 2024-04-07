pub mod jwk;

use josekit::JoseError;

/// Enum defining all supported cryptographic key types.
pub enum KeyType {
    Secp256k1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JoseError(#[from] JoseError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Failed to compute key thumbprint")]
    ThumprintFailed,
}

// TODO not sure this is necessary?
// /// Trait defining all common behavior for cryptographic keys.
// pub trait Key {
//     fn jwk(&self) -> &Jwk;
// }

// todo assuming only asymmetric encryption currently

pub trait PublicKey {
    /// Verifies a payload with a given signature using the target [`PublicKey`].
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError>;

    fn alias(&self) -> Result<String, KeyError>;
}

pub trait PrivateKey<T: PublicKey>: Sized {
    fn generate(key_type: KeyType) -> Result<Self, KeyError>;

    /// Derive a [`PublicKey`] from the target [`PrivateKey`].
    fn to_public(&self) -> Result<T, KeyError>;

    /// Sign a payload using the target [`PrivateKey`].
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError>;
}