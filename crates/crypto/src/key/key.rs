use ssi_jwk::JWK;
use ssi_jws::Error as JWSError;

/// Enum defining all supported cryptographic key types.
///
/// Note that KeyType is NOT to be interpreted as the "kty" header defined in https://www.rfc-editor.org/rfc/rfc7517#section-4.1.
/// Instead, "kty" (the JOSE header) should be thought of as the type for a Json Web Key (JWK)
/// object. This enum should be thought of as the type of a cryptographic key.
/// KeyType maps to a JWK type, but not the other way around. For example, the KeyType of "Ed25519"
/// maps to the JWK type (i.e. the "kty" header) of "EC". The converse is not true. "EC" maps to
/// many KeyType enums.
pub enum KeyType {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    #[error(transparent)]
    JWSError(#[from] JWSError),
    #[error("Algorithm not found on JWK")]
    AlgorithmNotFound,
}

/// Trait defining all common behavior for cryptographic keys.
pub trait Key {
    fn jwk(&self) -> &JWK;
}
