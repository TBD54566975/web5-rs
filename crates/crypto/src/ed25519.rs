use crate::{CryptoError, CurveOperations};
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
    SIGNATURE_LENGTH,
};
use jwk::Jwk;
use rand::rngs::OsRng;

pub struct Ed25199;

impl Ed25199 {}

impl CurveOperations for Ed25199 {
    fn generate() -> Result<Jwk, CryptoError> {
        let signing_key = SigningKey::generate(&mut OsRng {});
        let verifying_key = signing_key.verifying_key();

        let private_key_bytes = signing_key.to_bytes();
        let public_key_bytes = verifying_key.to_bytes();

        Ok(Jwk {
            alg: "EdDSA".to_string(),
            kty: "OKP".to_string(),
            crv: "Ed25519".to_string(),
            x: general_purpose::URL_SAFE_NO_PAD.encode(&public_key_bytes),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(&private_key_bytes)),
            ..Default::default()
        })
    }

    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let d = private_jwk
            .d
            .as_ref()
            .ok_or(CryptoError::MissingPrivateKey)?;
        let decoded_d = general_purpose::URL_SAFE_NO_PAD.decode(d)?;
        if decoded_d.len() != SECRET_KEY_LENGTH {
            return Err(CryptoError::InvalidKeyLength(SECRET_KEY_LENGTH.to_string()));
        }
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&decoded_d);
        let signing_key = SigningKey::from_bytes(&key_array);
        let signature = signing_key.sign(payload);
        Ok(signature.to_vec())
    }

    fn verify(public_jwk: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&public_jwk.x)?;

        if decoded_x.len() != PUBLIC_KEY_LENGTH {
            return Err(CryptoError::InvalidKeyLength(PUBLIC_KEY_LENGTH.to_string()));
        }

        let mut public_key_bytes = [0u8; PUBLIC_KEY_LENGTH];
        public_key_bytes.copy_from_slice(&decoded_x);
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|_| CryptoError::PublicKeyFailure(public_jwk.x.clone()))?;

        if signature.len() != SIGNATURE_LENGTH {
            return Err(CryptoError::InvalidSignatureLength(public_jwk.x.clone()));
        }

        let mut signature_bytes = [0u8; SIGNATURE_LENGTH];
        signature_bytes.copy_from_slice(signature);
        let verify_result = verifying_key.verify(payload, &Signature::from_bytes(&signature_bytes));

        match verify_result {
            Ok(_) => Ok(()),
            Err(e) => Err(CryptoError::VerificationFailure(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keys() {
        let jwk = Ed25199::generate().unwrap();
        assert_eq!(jwk.alg, "EdDSA");
        assert_eq!(jwk.kty, "OKP");
        assert_eq!(jwk.crv, "Ed25519");
        assert_eq!(jwk.x.len(), 43); // base64 URL-safe no padding length of 32 bytes
        assert_eq!(jwk.d.as_ref().unwrap().len(), 43); // base64 URL-safe no padding length of 32 bytes
    }

    #[test]
    fn test_sign_and_verify() {
        let jwk = Ed25199::generate().unwrap();
        let payload = b"test payload";
        let signature = Ed25199::sign(&jwk, payload).unwrap();

        assert!(Ed25199::verify(&jwk, payload, &signature).is_ok());
    }

    #[test]
    fn test_sign_with_invalid_private_key_length() {
        let mut jwk = Ed25199::generate().unwrap();
        let payload = b"another test payload";
        // Alter the private key to an invalid length
        jwk.d = Some(general_purpose::URL_SAFE_NO_PAD.encode(&[0u8; 31])); // one byte short

        assert!(Ed25199::sign(&jwk, payload).is_err());
    }

    #[test]
    fn test_verification_failure_with_invalid_public_key_length() {
        let jwk = Ed25199::generate().unwrap();
        let payload = b"test payload again";
        let signature = Ed25199::sign(&jwk, payload).unwrap();

        let mut jwk_modified = jwk.clone();
        // Alter the public key to an invalid length
        jwk_modified.x = general_purpose::URL_SAFE_NO_PAD.encode(&[0u8; 31]); // one byte short

        assert!(Ed25199::verify(&jwk_modified, payload, &signature).is_err());
    }

    #[test]
    fn test_verification_failure_with_modified_signature() {
        let jwk = Ed25199::generate().unwrap();
        let payload = b"yet another test payload";
        let mut signature = Ed25199::sign(&jwk, payload).unwrap();
        // Introduce an error in the signature
        signature[0] ^= 0xff;

        assert!(Ed25199::verify(&jwk, payload, &signature).is_err());
    }
}
