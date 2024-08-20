use crate::errors::{Result, Web5Error};
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
    pub(crate) fn is_public_key(&self) -> bool {
        self.d.is_none()
    }
}

impl Jwk {
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
        use crate::{errors::Web5Error, test_helpers::UnitTestSuite, test_name};
        use std::sync::LazyLock;

        static TEST_SUITE: LazyLock<UnitTestSuite> =
            LazyLock::new(|| UnitTestSuite::new("jwk_compute_thumbprint"));

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_ec_valid() {
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
            TEST_SUITE.include(test_name!());

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
