use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Jwk {
    pub alg: String,
    pub kty: String,
    pub crv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    pub x: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl From<Jwk> for josekit::jwk::Jwk {
    fn from(my_jwk: Jwk) -> Self {
        let mut map = Map::new();
        map.insert("alg".to_string(), Value::String(my_jwk.alg));
        map.insert("kty".to_string(), Value::String(my_jwk.kty));
        map.insert("crv".to_string(), Value::String(my_jwk.crv));
        if let Some(d) = my_jwk.d {
            map.insert("d".to_string(), Value::String(d));
        }
        map.insert("x".to_string(), Value::String(my_jwk.x));
        if let Some(y) = my_jwk.y {
            map.insert("y".to_string(), Value::String(y));
        };

        josekit::jwk::Jwk::from_map(map).unwrap()
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwkError {
    #[error("thumbprint computation failed {0}")]
    ThumbprintFailed(String),
}

impl Jwk {
    pub fn compute_thumbprint(&self) -> Result<String, JwkError> {
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

    pub fn to_public_jwk(&self) -> Jwk {
        Jwk {
            alg: self.alg.clone(),
            kty: self.kty.clone(),
            crv: self.crv.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_thumbprint() {
        let jwk = Jwk {
            alg: "".to_string(),
            kty: "EC".to_string(),
            crv: "secp256k1".to_string(),
            d: Some("".to_string()),
            x: "IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY".to_string(),
            y: Some("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA".to_string()),
        };

        let expected_thumbprint = "bgEObpJ1QzKa0jhWUkMSQKDOSDKEmwIw77ewaYpyduk";
        let computed_thumbprint = jwk
            .compute_thumbprint()
            .expect("Failed to compute thumbprint");

        assert_eq!(
            computed_thumbprint, expected_thumbprint,
            "Computed thumbprint does not match the expected value."
        );
    }
}
