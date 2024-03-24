use base64::{encode_config, URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct JWK {
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(rename = "crv", skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl JWK {
    pub fn new() -> Self {
        JWK {
            alg: None,
            kty: None,
            crv: None,
            d: None,
            x: None,
            y: None,
        }
    }

    pub fn compute_thumbprint(&self) -> String {
        let mut thumbprint_payload = serde_json::Map::new();
        if let Some(crv) = &self.crv {
            thumbprint_payload.insert("crv".to_string(), serde_json::Value::String(crv.clone()));
        }
        if let Some(kty) = &self.kty {
            thumbprint_payload.insert("kty".to_string(), serde_json::Value::String(kty.clone()));
        }
        if let Some(x) = &self.x {
            thumbprint_payload.insert("x".to_string(), serde_json::Value::String(x.clone()));
        }
        if let Some(y) = &self.y {
            thumbprint_payload.insert("y".to_string(), serde_json::Value::String(y.clone()));
        }

        match serde_json::to_vec(&thumbprint_payload) {
            Ok(bytes) => {
                let digest = Sha256::digest(&bytes);
                encode_config(&digest, URL_SAFE_NO_PAD)
            }
            Err(_) => {
                // Decide how you want to handle the error. For example, return a specific error string
                // or log the error for debugging purposes.
                // This is a placeholder; adapt it to your error handling policy.
                "Error computing thumbprint".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Adjust this line if your JWK struct and impl are in a different module

    #[test]
    fn test_compute_thumbprint() {
        let jwk = JWK {
            alg: Some("".to_string()),
            kty: Some("EC".to_string()),
            crv: Some("secp256k1".to_string()),
            d: Some("".to_string()),
            x: Some("IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY".to_string()),
            y: Some("UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA".to_string()),
        };

        let expected_thumbprint = "bgEObpJ1QzKa0jhWUkMSQKDOSDKEmwIw77ewaYpyduk";
        let computed_thumbprint = jwk.compute_thumbprint();

        assert_eq!(
            computed_thumbprint, expected_thumbprint,
            "Computed thumbprint does not match the expected value."
        );
    }
}
