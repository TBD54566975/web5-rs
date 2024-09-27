use crate::errors::{Result, Web5Error};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Jwk {
    /// The algorithm intended for use with the key (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,

    /// The key type, such as `EC` (Elliptic Curve) or `OKP` (Octet Key Pair).
    pub kty: String,

    /// The curve associated with the key, such as `P-256` for EC keys.
    pub crv: String,

    /// The private key component (optional). This is `None` for public keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,

    /// The `x` coordinate for elliptic curve keys, or the public key for OKP keys.
    pub x: String,

    /// The `y` coordinate for elliptic curve keys (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl Jwk {
    /// Determines if the JWK is a public key.
    ///
    /// Returns `true` if the JWK represents a public key (i.e., the private key component `d` is `None`).
    pub(crate) fn is_public_key(&self) -> bool {
        match &self.d {
            None => true,
            Some(d_value) => d_value.is_empty(),
        }
    }
}

impl Jwk {
    /// Computes the thumbprint of the JWK.
    ///
    /// A thumbprint is a cryptographic hash that uniquely identifies the JWK based on its key type, curve, and public key components.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - A base64url-encoded thumbprint, or an error if required fields are missing.
    ///
    /// # Errors
    ///
    /// Returns an error if `kty`, `x`, or `crv` fields are missing or empty, or if the `y` field is missing or empty for EC keys.
    pub fn compute_thumbprint(&self) -> Result<String> {
        if self.kty.is_empty() {
            return Err(Web5Error::DataMember("kty cannot be empty".to_string()));
        }

        if self.x.is_empty() {
            return Err(Web5Error::DataMember("x cannot be empty".to_string()));
        }

        if self.crv.is_empty() {
            return Err(Web5Error::DataMember("crv cannot be empty".to_string()));
        }

        let thumbprint_json_string = match self.kty.as_str() {
            "EC" => {
                let y = self
                    .y
                    .as_ref()
                    .ok_or(Web5Error::DataMember("missing y".to_string()))?;
                if y.is_empty() {
                    return Err(Web5Error::DataMember("y cannot be empty".to_string()));
                }

                format!(
                    r#"{{"crv":"{}","kty":"EC","x":"{}","y":"{}"}}"#,
                    self.crv, self.x, y,
                )
            }
            "OKP" => format!(r#"{{"crv":"{}","kty":"OKP","x":"{}"}}"#, self.crv, self.x),
            _ => {
                return Err(Web5Error::DataMember(format!(
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

#[cfg(test)]
mod tests {
    use super::*;

    mod compute_thumbprint {
        use super::*;

        #[test]
        fn test_ec_valid() {
            let jwk = Jwk {
                kty: "EC".to_string(),
                crv: "secp256k1".to_string(),
                x: "x_value".to_string(),
                y: Some("y_value".to_string()),
                ..Default::default()
            };

            let thumbprint = jwk.compute_thumbprint().unwrap();
            assert_eq!(thumbprint, "yiiszVT5Lwt6760MW19cHaJ61qJKIfe20sUW8dNxBv4");
        }

        #[test]
        fn test_okp_valid() {
            let jwk = Jwk {
                kty: "OKP".to_string(),
                crv: "Ed25519".to_string(),
                x: "x_value".to_string(),
                ..Default::default()
            };

            let thumbprint = jwk.compute_thumbprint().unwrap();
            assert_eq!(thumbprint, "nDMRVZm4lpedGjuJGO4y3YVJJ0krDF0aSz4KhlncDdI");
        }

        #[test]
        fn test_unsupported_kty() {
            let jwk = Jwk {
                kty: "RSA".to_string(),
                crv: "RS256".to_string(),
                x: "x_value".to_string(),
                y: Some("y_value".to_string()),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error kty not supported RSA");
        }

        #[test]
        fn test_empty_kty() {
            let jwk = Jwk {
                kty: "".to_string(),
                crv: "Ed25519".to_string(),
                x: "x_value".to_string(),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error kty cannot be empty");
        }

        #[test]
        fn test_empty_x() {
            let jwk = Jwk {
                kty: "OKP".to_string(),
                crv: "Ed25519".to_string(),
                x: "".to_string(),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error x cannot be empty");
        }

        #[test]
        fn test_empty_crv() {
            let jwk = Jwk {
                kty: "EC".to_string(),
                crv: "".to_string(),
                x: "x_value".to_string(),
                y: Some("y_value".to_string()),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error crv cannot be empty");
        }

        #[test]
        fn test_ec_missing_y() {
            let jwk = Jwk {
                kty: "EC".to_string(),
                crv: "P-256".to_string(),
                x: "x_value".to_string(),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error missing y");
        }

        #[test]
        fn test_ec_empty_y() {
            let jwk = Jwk {
                kty: "EC".to_string(),
                crv: "P-256".to_string(),
                x: "x_value".to_string(),
                y: Some("".to_string()),
                ..Default::default()
            };

            let err = jwk.compute_thumbprint().unwrap_err();
            assert!(matches!(err, Web5Error::DataMember(_)));
            assert_eq!(err.to_string(), "data member error y cannot be empty");
        }
    }
}
