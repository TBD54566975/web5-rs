mod jwk;
mod key;

pub use jwk::*;
pub use key::*;

#[derive(PartialEq, Debug)]
pub enum DidMethod {
    Key,
    Jwk,
}

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum DidMethodError {
    #[error("Unsupported DID method")]
    UnsupportedDidMethod,
}

impl std::str::FromStr for DidMethod {
    type Err = DidMethodError;

    fn from_str(s: &str) -> Result<Self, DidMethodError> {
        match s {
            "jwk" => Ok(DidMethod::Jwk),
            "key" => Ok(DidMethod::Key),
            _ => Err(DidMethodError::UnsupportedDidMethod),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_jwk_method_parsing() {
        assert_eq!(DidMethod::from_str("jwk"), Ok(DidMethod::Jwk));
    }

    #[test]
    fn test_key_method_parsing() {
        assert_eq!(DidMethod::from_str("key"), Ok(DidMethod::Key));
    }

    #[test]
    fn test_invalid_method_parsing() {
        assert_eq!(
            DidMethod::from_str("invalid"),
            Err(DidMethodError::UnsupportedDidMethod)
        );
    }
}
