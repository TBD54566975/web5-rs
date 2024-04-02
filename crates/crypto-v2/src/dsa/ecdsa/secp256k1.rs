use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jwk::jwk::JWK;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use super::KEY_TYPE;

pub const SECP256K1_JWA: &str = "ES256K";
pub const SECP256K1_JWA_CURVE: &str = "secp256k1";
pub const SECP256K1_ALGORITHM_ID: &str = SECP256K1_JWA_CURVE;

pub fn secp256k1_generate_key_pair() -> Result<JWK, String> {
    let (secret_key, public_key) = Secp256k1::new().generate_keypair(&mut OsRng);

    let d_bytes = secret_key.secret_bytes();

    let public_key_serialized = public_key.serialize_uncompressed();
    let x_bytes = &public_key_serialized[1..33];
    let y_bytes = &public_key_serialized[33..65];

    let private_key = JWK {
        kty: Some(String::from(KEY_TYPE)),
        crv: Some(String::from(SECP256K1_JWA_CURVE)),
        d: Some(URL_SAFE_NO_PAD.encode(d_bytes)),
        x: Some(URL_SAFE_NO_PAD.encode(&x_bytes)),
        y: Some(URL_SAFE_NO_PAD.encode(&y_bytes)),
        ..Default::default()
    };

    Ok(private_key)
}

pub fn secp256k1_sign(payload: &[u8], jwk: &JWK) -> Result<Vec<u8>, String> {
    let d_bytes = URL_SAFE_NO_PAD
        .decode(jwk.d.as_ref().ok_or("Missing 'd' in JWK")?)
        .map_err(|e| format!("Failed to decode 'd': {}", e))?;
    let secret_key = SecretKey::from_slice(&d_bytes)
        .map_err(|e| format!("Failed to create secret key from 'd': {}", e))?;

    let mut hasher = Sha256::new();
    hasher.update(payload);
    let hash_result = hasher.finalize();
    let message = Message::from_digest_slice(&hash_result)
        .map_err(|e| format!("Failed to create message from payload hash: {}", e))?;

    let secp = Secp256k1::signing_only();
    let signature = secp.sign_ecdsa(&message, &secret_key);
    let signature_compact = signature.serialize_compact();

    Ok(signature_compact.to_vec())
}

pub fn secp256k1_verify(payload: &[u8], signature_bytes: &[u8], jwk: &JWK) -> Result<bool, String> {
    let x_bytes = URL_SAFE_NO_PAD
        .decode(jwk.x.as_ref().ok_or("Missing 'x' in JWK")?)
        .map_err(|e| format!("Failed to decode 'x': {}", e))?;
    let y_bytes = URL_SAFE_NO_PAD
        .decode(jwk.y.as_ref().ok_or("Missing 'y' in JWK")?)
        .map_err(|e| format!("Failed to decode 'y': {}", e))?;

    let mut public_key_data = Vec::with_capacity(1 + x_bytes.len() + y_bytes.len());
    public_key_data.push(0x04); // Uncompressed public key prefix
    public_key_data.extend_from_slice(&x_bytes);
    public_key_data.extend_from_slice(&y_bytes);

    let public_key = PublicKey::from_slice(&public_key_data)
        .map_err(|e| format!("Failed to create public key: {}", e))?;

    let mut hasher = Sha256::new();
    hasher.update(payload);
    let hash_result = hasher.finalize();

    let message = Message::from_digest_slice(&hash_result)
        .map_err(|e| format!("Failed to create message from payload hash: {}", e))?;

    let signature = Signature::from_compact(signature_bytes)
        .map_err(|e| format!("Failed to deserialize signature: {}", e))?;

    let secp = Secp256k1::verification_only();
    match secp.verify_ecdsa(&message, &signature, &public_key) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secp256k1_generate_private_key() {
        let jwk_result = secp256k1_generate_key_pair();
        assert!(jwk_result.is_ok());

        let jwk = jwk_result.unwrap();

        assert_eq!(jwk.kty, Some(String::from(KEY_TYPE)));
        assert_eq!(jwk.crv, Some(String::from(SECP256K1_JWA_CURVE)));

        assert!(jwk.d.is_some(), "JWK.d is missing");
        assert!(jwk.x.is_some(), "JWK.x is missing");
        assert!(jwk.y.is_some(), "JWK.y is missing");

        assert!(
            URL_SAFE_NO_PAD.decode(jwk.d.as_ref().unwrap()).is_ok(),
            "JWK.d incorrectly encoded"
        );
        assert!(
            URL_SAFE_NO_PAD.decode(jwk.x.as_ref().unwrap()).is_ok(),
            "JWK.x incorrectly encoded"
        );
        assert!(
            URL_SAFE_NO_PAD.decode(jwk.y.as_ref().unwrap()).is_ok(),
            "JWK.y incorrectly encoded"
        );
    }

    #[test]
    fn test_secp256k1_sign() {
        let jwk = secp256k1_generate_key_pair().unwrap();
        let payload = b"example payload";

        let signature_result = secp256k1_sign(payload, &jwk);
        assert!(signature_result.is_ok());
    }

    #[test]
    fn test_secp256k1_sign_and_verify() {
        let jwk = secp256k1_generate_key_pair().expect("Failed to generate key pair");

        let payload = b"Hello, SECP256K1!";

        let signature = secp256k1_sign(payload, &jwk)
            .expect("Failed to sign payload");

        let verification_result = secp256k1_verify(payload, &signature, &jwk)
            .expect("Verification process failed");

        assert!(verification_result, "Failed to verify signature");
    }
}
