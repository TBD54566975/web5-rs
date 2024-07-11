use super::{DsaError, Result, Signer, Verifier};
use crate::crypto::jwk::Jwk;
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{
    Signature, Signer as DalekSigner, SigningKey, Verifier as DalekVerifier, VerifyingKey,
    PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH,
};
use rand::rngs::OsRng;

pub struct Ed25519Generator;

impl Ed25519Generator {
    pub fn generate() -> Jwk {
        let signing_key = SigningKey::generate(&mut OsRng {});
        let verifying_key = signing_key.verifying_key();

        let private_key_bytes = signing_key.to_bytes();
        let public_key_bytes = verifying_key.to_bytes();

        Jwk {
            alg: Some("Ed25519".to_string()),
            kty: "OKP".to_string(),
            crv: "Ed25519".to_string(),
            x: general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes),
            d: Some(general_purpose::URL_SAFE_NO_PAD.encode(private_key_bytes)),
            ..Default::default()
        }
    }
}

pub(crate) fn public_jwk_from_bytes(public_key: &[u8]) -> Result<Jwk> {
    if public_key.len() != PUBLIC_KEY_LENGTH {
        return Err(DsaError::PublicKeyFailure(format!(
            "Public key has incorrect length {}",
            PUBLIC_KEY_LENGTH
        )));
    }

    Ok(Jwk {
        alg: Some("Ed25519".to_string()),
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        x: general_purpose::URL_SAFE_NO_PAD.encode(public_key),
        ..Default::default()
    })
}

#[cfg(test)]
pub fn to_public_jwk(jwk: &Jwk) -> Jwk {
    Jwk {
        alg: jwk.alg.clone(),
        kty: jwk.kty.clone(),
        crv: jwk.crv.clone(),
        x: jwk.x.clone(),
        y: jwk.y.clone(),
        ..Default::default()
    }
}

pub(crate) fn public_jwk_extract_bytes(jwk: &Jwk) -> Result<Vec<u8>> {
    let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&jwk.x)?;

    if decoded_x.len() != PUBLIC_KEY_LENGTH {
        return Err(DsaError::InvalidKeyLength(PUBLIC_KEY_LENGTH.to_string()));
    }

    Ok(decoded_x)
}

#[derive(Clone)]
pub struct Ed25519Signer {
    private_jwk: Jwk,
}

impl Ed25519Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self { private_jwk }
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        let d = self
            .private_jwk
            .d
            .as_ref()
            .ok_or(DsaError::MissingPrivateKey)?;
        let decoded_d = general_purpose::URL_SAFE_NO_PAD.decode(d)?;

        // some implementations of ed25519 couple the public key alongside the private key
        // in which case, we need to splice out only the private key bytes
        let signing_key = if decoded_d.len() == 64 {
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&decoded_d[..32]);
            SigningKey::from_bytes(&key_array)
        } else if decoded_d.len() == 32 {
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&decoded_d);
            SigningKey::from_bytes(&key_array)
        } else {
            return Err(DsaError::InvalidKeyLength(SECRET_KEY_LENGTH.to_string()));
        };

        let signature = signing_key.sign(payload);
        Ok(signature.to_vec())
    }
}

#[derive(Clone)]
pub struct Ed25519Verifier {
    public_jwk: Jwk,
}

impl Ed25519Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self { public_jwk }
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool> {
        let mut public_key_bytes = [0u8; PUBLIC_KEY_LENGTH];
        let decoded_x = general_purpose::URL_SAFE_NO_PAD.decode(&self.public_jwk.x)?;

        if decoded_x.len() != PUBLIC_KEY_LENGTH {
            return Err(DsaError::InvalidKeyLength(PUBLIC_KEY_LENGTH.to_string()));
        }

        public_key_bytes.copy_from_slice(&decoded_x);
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|e| DsaError::PublicKeyFailure(e.to_string()))?;

        if signature.len() != SIGNATURE_LENGTH {
            return Err(DsaError::InvalidSignatureLength(self.public_jwk.x.clone()));
        }

        let mut signature_bytes = [0u8; SIGNATURE_LENGTH];
        signature_bytes.copy_from_slice(signature);
        let verify_result = verifying_key.verify(payload, &Signature::from_bytes(&signature_bytes));

        match verify_result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
