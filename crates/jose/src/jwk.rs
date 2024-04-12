use base64::{engine::general_purpose, Engine as _};
use josekit::{
    jwk::{
        alg::{ec::EcCurve as JosekitEcCurve, ed::EdCurve as JosekitEdCurve},
        Jwk as JosekitJwk,
    },
    jws::{
        alg::{
            ecdsa::EcdsaJwsAlgorithm as JosekitEcdsaJwsAlgorithm,
            eddsa::EddsaJwsAlgorithm as JosekitEddsaJwsAlgorithm,
        },
        JwsSigner as JosekitJwsSigner, JwsVerifier as JosekitJwsVerifier,
    },
    JoseError as JosekitJoseError,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwkError {
    #[error("{0}")]
    JoseError(String),
    #[error("curve not found on JWK")]
    CurveNotFound,
    #[error("algorithm not found on JWK")]
    AlgorithmNotFound,
    #[error("failed to compute key thumbprint missing {0}")]
    ThumprintFailed(String),
}

pub enum Curve {
    Secp256k1,
    Ed25519,
}

impl From<JosekitJoseError> for JwkError {
    fn from(error: JosekitJoseError) -> Self {
        JwkError::JoseError(error.to_string())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Jwk(pub(crate) JosekitJwk);

impl Jwk {
    pub fn parameter(&self, p: &str) -> Option<&Value> {
        self.0.parameter(p)
    }

    pub fn set_signing_properties(&mut self, curve: Curve) -> Result<(), JwkError> {
        // key_id and alg needed for signing with the JwsSigner
        let key_alias = self.thumbprint()?;
        self.0.set_key_id(&key_alias);

        self.0.set_algorithm(match curve {
            Curve::Secp256k1 => JosekitEcdsaJwsAlgorithm::Es256k.to_string(),
            Curve::Ed25519 => JosekitEddsaJwsAlgorithm::Eddsa.to_string(),
        });

        Ok(())
    }

    pub fn generate_private_key(curve: Curve) -> Result<Self, JwkError> {
        let mut jwk = Self(match curve {
            Curve::Secp256k1 => JosekitJwk::generate_ec_key(JosekitEcCurve::Secp256k1),
            Curve::Ed25519 => JosekitJwk::generate_ed_key(JosekitEdCurve::Ed25519),
        }?);

        jwk.set_signing_properties(curve)?;

        Ok(jwk)
    }

    pub fn thumbprint(&self) -> Result<String, JwkError> {
        let thumbprint_json_string = match self.0.key_type() {
            "EC" => format!(
                r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
                self.0
                    .curve()
                    .ok_or(JwkError::ThumprintFailed("curve".to_string()))?,
                self.0
                    .parameter("x")
                    .ok_or(JwkError::ThumprintFailed("x".to_string()))?,
                self.0
                    .parameter("y")
                    .ok_or(JwkError::ThumprintFailed("y".to_string()))?,
            ),
            "OKP" => format!(
                r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#,
                self.0
                    .curve()
                    .ok_or(JwkError::ThumprintFailed("curve".to_string()))?,
                self.0
                    .parameter("x")
                    .ok_or(JwkError::ThumprintFailed("x".to_string()))?,
            ),
            _ => return Err(JwkError::ThumprintFailed("curve".to_string())),
        };
        let mut hasher = Sha256::new();
        hasher.update(thumbprint_json_string);
        let digest = hasher.finalize();
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(digest);

        Ok(thumbprint)
    }

    pub fn to_public(&self) -> Result<Self, JwkError> {
        let mut public_key = Self(self.0.to_public_key()?);
        public_key.set_signing_properties(match self.0.curve() {
            Some("secp256k1") => Curve::Secp256k1,
            Some("Ed25519") => Curve::Ed25519,
            _ => return Err(JwkError::CurveNotFound),
        })?;

        Ok(public_key)
    }

    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>, JwkError> {
        let signer: Box<dyn JosekitJwsSigner> = match self.0.curve() {
            Some("secp256k1") => {
                Box::new(JosekitEcdsaJwsAlgorithm::Es256k.signer_from_jwk(&self.0)?)
            }
            Some("Ed25519") => Box::new(JosekitEddsaJwsAlgorithm::Eddsa.signer_from_jwk(&self.0)?),
            _ => return Err(JwkError::CurveNotFound),
        };

        let signature = signer.sign(payload)?;

        Ok(signature)
    }

    pub fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<(), JwkError> {
        let verifier: Box<dyn JosekitJwsVerifier> = match self.0.curve() {
            Some("secp256k1") => {
                Box::new(JosekitEcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&self.0)?)
            }
            Some("Ed25519") => {
                Box::new(JosekitEddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&self.0)?)
            }
            _ => return Err(JwkError::CurveNotFound),
        };

        verifier.verify(payload, signature)?;
        Ok(())
    }

    pub fn algorithm(&self) -> Result<String, JwkError> {
        match self.0.algorithm() {
            Some(alg) => Ok(alg.to_string()),
            None => Err(JwkError::AlgorithmNotFound),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, JwkError> {
        let josekit_jwk = serde_json::from_slice::<JosekitJwk>(bytes)
            .map_err(|err| JwkError::JoseError(err.to_string()))?;
        Ok(Jwk(josekit_jwk))
    }
}

impl Serialize for Jwk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Directly serialize self.0, which is of type JosekitJwk
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Jwk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Directly deserialize into JosekitJwk and wrap it in Jwk
        let josekit_jwk = JosekitJwk::deserialize(deserializer)?;
        Ok(Jwk(josekit_jwk))
    }
}
