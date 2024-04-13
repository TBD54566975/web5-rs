use crate::{CryptoError, CurveOperations};
use base64::{engine::general_purpose, Engine as _};
use jwk::Jwk;
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};

pub struct Secp256k1;

impl CurveOperations for Secp256k1 {
    fn generate() -> Result<Jwk, CryptoError> {
        let signing_key = SigningKey::random(&mut rand::thread_rng());
        let verifying_key = signing_key.verifying_key();
        let serialized_pub_key = verifying_key.to_encoded_point(false);
        let bytes = serialized_pub_key.as_bytes();
        let x_bytes = &bytes[1..33];
        let y_bytes = &bytes[33..65];

        Ok(Jwk {
            alg: "ES256K".to_string(),
            kty: "EC".to_string(),
            crv: "secp256k1".to_string(),
            x: general_purpose::URL_SAFE_NO_PAD.encode(x_bytes),
            y: Some(general_purpose::URL_SAFE_NO_PAD.encode(y_bytes)),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(signing_key.to_bytes().as_slice())),
        })
    }

    fn sign(private_jwk: &Jwk, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let d = private_jwk
            .d
            .as_ref()
            .ok_or(CryptoError::MissingPrivateKey)?;
        let decoded_d = general_purpose::URL_SAFE_NO_PAD.decode(d)?;
        let signing_key = SigningKey::from_slice(&decoded_d)
            .map_err(|e| CryptoError::PrivateKeyFailure(e.to_string()))?;
        let signature: Signature = signing_key.sign(payload);
        Ok(signature.to_vec())
    }

    fn verify(public_jwk: &Jwk, payload: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&public_jwk.x)?;
        let decoded_y = general_purpose::URL_SAFE_NO_PAD.decode(
            &public_jwk
                .y
                .as_ref()
                .ok_or(CryptoError::PublicKeyFailure("missing y".to_string()))?,
        )?;

        let mut pk_bytes = Vec::with_capacity(1 + decoded_x.len() + decoded_y.len());
        pk_bytes.push(0x04); // Uncompressed point indicator
        pk_bytes.extend_from_slice(&decoded_x);
        pk_bytes.extend_from_slice(&decoded_y);
        let encoded_point = EncodedPoint::from_bytes(&pk_bytes)
            .map_err(|e| CryptoError::PublicKeyFailure(e.to_string()))?;

        let verifying_key = VerifyingKey::from_encoded_point(&encoded_point)
            .map_err(|e| CryptoError::PublicKeyFailure(e.to_string()))?;

        let s: Signature = Signature::from_bytes(signature.into())
            .map_err(|e| CryptoError::VerificationFailure(e.to_string()))?;

        let _ = verifying_key
            .verify(payload, &s)
            .map_err(|e| CryptoError::VerificationFailure(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keys() {
        let jwk = Secp256k1::generate().unwrap();
        assert_eq!(jwk.alg, "ES256K");
        assert_eq!(jwk.kty, "EC");
        assert_eq!(jwk.crv, "secp256k1");
        assert!(jwk.x.len() > 0);
        assert!(jwk.y.as_ref().unwrap().len() > 0);
        assert!(jwk.d.as_ref().unwrap().len() > 0);
    }

    #[test]
    fn test_sign_and_verify() {
        let jwk = Secp256k1::generate().unwrap();
        let payload = b"hello world";
        let signature = Secp256k1::sign(&jwk, payload).unwrap();

        assert!(Secp256k1::verify(&jwk, payload, &signature).is_ok());
    }

    #[test]
    fn test_verification_failure_on_modified_payload() {
        let jwk = Secp256k1::generate().unwrap();
        let payload = b"hello world";
        let signature = Secp256k1::sign(&jwk, payload).unwrap();
        let modified_payload = b"hello mars";

        assert!(Secp256k1::verify(&jwk, modified_payload, &signature).is_err());
    }

    #[test]
    fn test_verification_failure_on_modified_signature() {
        let jwk = Secp256k1::generate().unwrap();
        let payload = b"hello world";
        let mut signature = Secp256k1::sign(&jwk, payload).unwrap();
        // Introduce an error in the signature
        signature[0] ^= 0xff;

        assert!(Secp256k1::verify(&jwk, payload, &signature).is_err());
    }
}
