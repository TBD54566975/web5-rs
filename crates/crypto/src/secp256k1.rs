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
