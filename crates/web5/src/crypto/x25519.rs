use base64::{engine::general_purpose, Engine as _};
use x25519_dalek::{PublicKey, StaticSecret};

use crate::{crypto::jwk::Jwk, errors::Result, errors::Web5Error};

const PUBLIC_KEY_LENGTH: usize = 32;

pub struct X25519Generator;

impl X25519Generator {
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

pub(crate) fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let x_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(&jwk.x)
        .map_err(|e| Web5Error::Parameter(e.to_string()))?;

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