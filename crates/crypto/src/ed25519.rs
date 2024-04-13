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
