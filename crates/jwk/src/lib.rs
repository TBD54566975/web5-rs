use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Jwk {
    pub alg: String,
    pub kty: String,
    pub crv: String,
    pub d: Option<String>,
    pub x: String,
    pub y: Option<String>,
}

impl Jwk {
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
    use super::*; // Adjust this line if your JWK struct and impl are in a different module

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
