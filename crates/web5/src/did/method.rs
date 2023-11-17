pub mod did_jwk;
pub mod did_key;

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
