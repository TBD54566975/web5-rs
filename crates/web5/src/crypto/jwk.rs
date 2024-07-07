use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Jwk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    pub kty: String,
    pub crv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    pub x: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl Jwk {
    pub fn is_private_key(&self) -> bool {
        self.d.is_some()
    }

    pub fn is_public_key(&self) -> bool {
        self.d.is_none()
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwkError {
    #[error("thumbprint computation failed {0}")]
    ThumbprintFailed(String),
}

type Result<T> = std::result::Result<T, JwkError>;

impl Jwk {
    pub fn compute_thumbprint(&self) -> Result<String> {
        let thumbprint_json_string = match self.kty.as_str() {
            "EC" => format!(
                r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
                self.crv,
                self.x,
                self.y
                    .as_ref()
                    .ok_or(JwkError::ThumbprintFailed("missing y".to_string()))?,
            ),
            "OKP" => format!(r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#, self.crv, self.x,),
            _ => {
                return Err(JwkError::ThumbprintFailed(format!(
                    "kty not supported {0}",
                    self.kty
                )))
            }
        };
        let mut hasher = Sha256::new();
        hasher.update(thumbprint_json_string);

        let digest = hasher.finalize();
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(digest);

        Ok(thumbprint)
    }
}
