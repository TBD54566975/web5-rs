use super::{Curve, Key, KeyError, PrivateKey, PublicKey};
use base64::{engine::general_purpose, Engine as _};
use josekit::{
    jwk::{
        alg::{ec::EcCurve, ed::EdCurve},
        Jwk,
    },
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsSigner, JwsVerifier,
    },
    JoseError,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

impl From<JoseError> for KeyError {
    fn from(error: JoseError) -> Self {
        KeyError::JoseError(error.to_string())
    }
}

pub fn generate_private_jwk(curve: Curve) -> Result<Arc<Jwk>, KeyError> {
    let mut jwk = match curve {
        Curve::Secp256k1 => Jwk::generate_ec_key(EcCurve::Secp256k1),
        Curve::Ed25519 => Jwk::generate_ed_key(EdCurve::Ed25519),
    }?;

    // key_id and alg needed for signing with the JwsSigner
    let key_alias = jwk.alias()?;
    jwk.set_key_id(&key_alias);
    jwk.set_algorithm(match curve {
        Curve::Secp256k1 => EcdsaJwsAlgorithm::Es256k.to_string(),
        Curve::Ed25519 => EddsaJwsAlgorithm::Eddsa.to_string(),
    });

    Ok(Arc::new(jwk))
}

impl Key for Jwk {
    fn alias(&self) -> Result<String, KeyError> {
        let thumbprint_json_string = match self.key_type() {
            "EC" => format!(
                r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
                self.curve()
                    .ok_or(KeyError::ThumprintFailed("missing curve".to_string()))?,
                self.parameter("x")
                    .ok_or(KeyError::ThumprintFailed("missing x".to_string()))?,
                self.parameter("y")
                    .ok_or(KeyError::ThumprintFailed("missing y".to_string()))?,
            ),
            "OKP" => format!(
                r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#,
                self.curve()
                    .ok_or(KeyError::ThumprintFailed("missing curve".to_string()))?,
                self.parameter("x")
                    .ok_or(KeyError::ThumprintFailed("missing x".to_string()))?,
            ),
            _ => return Err(KeyError::ThumprintFailed("missing curve".to_string())),
        };
        let mut hasher = Sha256::new();
        hasher.update(thumbprint_json_string);
        let digest = hasher.finalize();
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(digest);

        Ok(thumbprint)
    }

    fn jwk(&self) -> Result<Jwk, KeyError> {
        Ok(self.clone())
    }
}

impl PublicKey for Jwk {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), KeyError> {
        let verifier: Box<dyn JwsVerifier> = match self.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&self)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&self)?),
            _ => return Err(KeyError::CurveNotFound),
        };

        verifier.verify(payload, signature)?;
        Ok(())
    }

    fn algorithm(&self) -> Result<String, KeyError> {
        match self.algorithm() {
            Some(alg) => Ok(alg.to_string()),
            None => Err(KeyError::AlgorithmNotFound),
        }
    }

    fn to_json(&self) -> Result<String, KeyError> {
        let json_str = serde_json::to_string(self).map_err(|_| KeyError::SerializationFailed)?;
        Ok(json_str)
    }
}

impl PrivateKey for Jwk {
    fn to_public(&self) -> Result<Arc<dyn PublicKey>, KeyError> {
        let public_key = self.to_public_key()?;
        Ok(Arc::new(public_key))
    }

    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, KeyError> {
        let signer: Box<dyn JwsSigner> = match self.curve() {
            Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.signer_from_jwk(&self)?),
            Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.signer_from_jwk(&self)?),
            _ => return Err(KeyError::CurveNotFound),
        };

        let signature = signer.sign(payload)?;

        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        let private_key = generate_private_jwk(Curve::Ed25519.into()).unwrap();
        let payload = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }

    #[test]
    fn test_verify_failure() {
        let private_key = generate_private_jwk(Curve::Ed25519.into()).unwrap();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        // public_key is unrelated to the private_key used to sign the payload, so it should fail
        let private_key_2 = generate_private_jwk(Curve::Ed25519.into()).unwrap();
        let public_key = private_key_2.to_public().unwrap();
        let verification_warnings = public_key.verify(payload, &signature);
        assert!(verification_warnings.is_err());
    }

    #[test]
    fn test_to_public() {
        let private_key = generate_private_jwk(Curve::Ed25519.into()).unwrap();
        let public_key = private_key.to_public().unwrap().jwk().unwrap();

        assert_eq!(private_key.parameter("x"), public_key.parameter("x"));
        assert_eq!(private_key.parameter("y"), public_key.parameter("y"));

        assert!(private_key.parameter("d").is_some());
        assert!(public_key.parameter("d").is_none());
    }

    #[test]
    fn test_sign() {
        let private_key = generate_private_jwk(Curve::Ed25519.into()).unwrap();
        let payload: &[u8] = b"hello world";
        let signature = private_key.sign(payload).unwrap();

        let public_key = private_key.to_public().unwrap();
        assert!(public_key.verify(payload, &signature).is_ok());
    }
}
