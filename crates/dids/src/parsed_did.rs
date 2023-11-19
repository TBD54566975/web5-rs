use crate::method::{DidMethod, DidMethodError}; // TODO: move this to mod?
use std::str::FromStr;

#[derive(Debug)]
pub struct ParsedDid {
    pub uri: String,
    pub method: DidMethod,
    pub method_id: String,
}

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum ParsedDidError {
    #[error("Provided DID URI is invalid")]
    InvalidDidUri,
    #[error(transparent)]
    DidMethodError(#[from] DidMethodError),
}

impl FromStr for ParsedDid {
    type Err = ParsedDidError;

    fn from_str(did_uri: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = did_uri.splitn(3, ':').collect();
        if parts.len() == 3 && parts[0] == "did" {
            Ok(Self {
                uri: did_uri.to_string(),
                method: DidMethod::from_str(parts[1])?,
                method_id: parts[2].to_string(),
            })
        } else {
            Err(ParsedDidError::InvalidDidUri)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_jwk_parsing() {
        let did_uri = "did:jwk:123123";
        let parsed_did = ParsedDid::from_str(did_uri).expect("Failed to parse DID");

        assert_eq!(parsed_did.uri, did_uri);
        assert_eq!(parsed_did.method, DidMethod::Jwk);
        assert_eq!(parsed_did.method_id, "123123");
    }

    #[test]
    fn test_did_key_parsing() {
        let did_uri = "did:key:123123";
        let parsed_did = ParsedDid::from_str(did_uri).expect("Failed to parse DID");

        assert_eq!(parsed_did.uri, did_uri);
        assert_eq!(parsed_did.method, DidMethod::Key);
        assert_eq!(parsed_did.method_id, "123123");
    }

    #[test]
    fn test_unsupported_method() {
        let did_uri = "did:unsupported:123123";
        let error = ParsedDid::from_str(did_uri).expect_err("Expected an error");

        assert_eq!(
            error,
            ParsedDidError::DidMethodError(DidMethodError::UnsupportedDidMethod)
        );
    }

    #[test]
    fn test_invalid_did_uri() {
        let invalid_did_uri = "wrong:did:key:123123";
        let error = ParsedDid::from_str(invalid_did_uri).expect_err("Expected an error");
        assert_eq!(error, ParsedDidError::InvalidDidUri);
    }
}
