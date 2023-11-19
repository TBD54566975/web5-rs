mod jwk;
mod key;
mod web;

use crypto::key_manager::KeyManagerError;
pub use jwk::*;
pub use key::*;
pub use web::*;

#[derive(PartialEq, Debug)]
pub enum DidMethod {
    Key,
    Jwk,
    Web,
}

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum DidMethodError {
    #[error("Unsupported DID method")]
    UnsupportedDidMethod,
}

#[derive(thiserror::Error, Debug)]
pub enum DidCreationError {
    #[error(transparent)]
    KeyManagerError(#[from] KeyManagerError),
    #[error("Did generation failed")]
    DidGenerationFailed,
}

impl std::str::FromStr for DidMethod {
    type Err = DidMethodError;

    fn from_str(s: &str) -> Result<Self, DidMethodError> {
        match s {
            "jwk" => Ok(DidMethod::Jwk),
            "key" => Ok(DidMethod::Key),
            "web" => Ok(DidMethod::Web),
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
