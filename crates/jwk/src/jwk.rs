use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JWK {
    pub alg: Option<String>,
    pub kty: Option<String>,
    pub crv: Option<String>,
    pub d: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
}

impl JWK {
    pub fn compute_thumbprint(&self) -> Result<String, serde_json::Error> {
        let thumbprint_payload = serde_json::json!({
            "crv": self.crv,
            "kty": self.kty,
            "x": self.x,
            "y": self.y,
        });

        let bytes = serde_json::to_vec(&thumbprint_payload)?;

        let digest = Sha256::digest(&bytes);
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(&digest);

        Ok(thumbprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let computed_thumbprint = jwk
            .compute_thumbprint()
            .expect("Failed to compute thumbprint");

        assert_eq!(
            computed_thumbprint, expected_thumbprint,
            "Computed thumbprint does not match the expected value."
        );
    }
}
