use base64::{engine::general_purpose, Engine as _};
use x25519_dalek::{PublicKey, StaticSecret};

use crate::{crypto::jwk::Jwk, errors::Result, errors::Web5Error};

const PUBLIC_KEY_LENGTH: usize = 32;

pub struct X25519Generator;

impl X25519Generator {
    /// Generates a new X25519 key pair and returns it as a JWK.
    ///
    /// The method creates a new random private key and derives the public key from it. The private
    /// key (`d`) and the public key (`x`) are encoded in base64url format, and a JWK is returned with
    /// the corresponding cryptographic parameters.
    ///
    /// # Returns
    /// A `Jwk` object containing the generated X25519 key pair.
    pub fn generate() -> Jwk {
        let private_key = StaticSecret::random();
        let public_key = PublicKey::from(&private_key);
        let x = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());
        let d = general_purpose::URL_SAFE_NO_PAD.encode(private_key.as_bytes());

        Jwk {
            alg: Some("ECDH-ES+A256KW".to_string()),
            kty: "OKP".to_string(),
            crv: "X25519".to_string(),
            d: Some(d),
            x,
            y: None,
        }
    }
}

/// Extracts the public key bytes from an X25519 JWK.
///
/// This function decodes the base64url-encoded `x` value of a JWK into its raw byte representation.
///
/// # Arguments
/// * `jwk` - The JWK from which to extract the public key.
///
/// # Returns
/// A `Result` containing a vector of bytes representing the public key, or an error if the key length is incorrect.
pub(crate) fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let x_bytes = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;

    if x_bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(Web5Error::Parameter(format!(
            "Public key has incorrect length {}",
            PUBLIC_KEY_LENGTH
        )));
    }

    let mut public_key_bytes = [0u8; 32];
    public_key_bytes.copy_from_slice(&x_bytes);

    Ok(public_key_bytes.to_vec())
}

/// Creates a JWK from raw X25519 public key bytes.
///
/// This function takes the raw bytes of an X25519 public key and constructs a corresponding JWK. The
/// public key is encoded in base64url format and stored in the `x` field of the JWK.
///
/// # Arguments
/// * `public_key_bytes` - A byte slice containing the raw public key.
///
/// # Returns
/// A `Result` containing the constructed JWK, or an error if the key length is incorrect.
pub(crate) fn public_jwk_from_bytes(public_key_bytes: &[u8]) -> Result<Jwk> {
    if public_key_bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(Web5Error::Parameter(format!(
            "Public key has incorrect length {}",
            PUBLIC_KEY_LENGTH
        )));
    }

    // Encode the public key bytes to a base64 URL-safe string
    let x = general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes);

    // Create the JWK
    Ok(Jwk {
        alg: None,
        kty: "OKP".to_string(),
        crv: "X25519".to_string(),
        d: None,
        x,
        y: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate {
        use super::*;

        #[test]
        fn test_must_set_alg() {
            let jwk = X25519Generator::generate();
            assert_eq!(jwk.alg, Some("ECDH-ES+A256KW".to_string()));
        }

        #[test]
        fn test_must_set_kty() {
            let jwk = X25519Generator::generate();
            assert_eq!(jwk.kty, "OKP".to_string());
        }

        #[test]
        fn test_must_set_crv() {
            let jwk = X25519Generator::generate();
            assert_eq!(jwk.crv, "X25519");
        }

        #[test]
        fn test_must_set_public_key_with_correct_length() {
            let jwk = X25519Generator::generate();
            let public_key_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(&jwk.x)
                .expect("Failed to decode public key");
            assert_eq!(public_key_bytes.len(), PUBLIC_KEY_LENGTH);
        }

        #[test]
        fn test_must_set_private_key_with_correct_length() {
            let jwk = X25519Generator::generate();
            let private_key_bytes = jwk.d.expect("Private key is missing");
            let decoded_private_key_bytes = general_purpose::URL_SAFE_NO_PAD
                .decode(private_key_bytes)
                .expect("Failed to decode private key");
            assert_eq!(decoded_private_key_bytes.len(), PUBLIC_KEY_LENGTH);
        }
    }
}
