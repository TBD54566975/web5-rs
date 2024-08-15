use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::PUBLIC_KEY_LENGTH;

use super::{DsaError, Result};
use crate::crypto::jwk::Jwk;

pub fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let x_bytes = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;

    if x_bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(DsaError::InvalidKeyLength(PUBLIC_KEY_LENGTH.to_string()));
    }

    let mut public_key_bytes = [0u8; 32];
    public_key_bytes.copy_from_slice(&x_bytes);

    Ok(public_key_bytes.to_vec())
}

pub fn public_jwk_from_bytes(public_key_bytes: &[u8]) -> Result<Jwk> {
    if public_key_bytes.len() != PUBLIC_KEY_LENGTH {
        return Err(DsaError::PublicKeyFailure(format!(
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
