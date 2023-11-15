pub mod did_jwk;
pub mod did_key;

pub enum DidMethod {
    Key,
    Jwk,
}

#[derive(thiserror::Error, Debug)]
#[error("Unsupported DID method")]
pub struct UnsupportedDidMethodError;

impl std::str::FromStr for DidMethod {
    type Err = UnsupportedDidMethodError;

    fn from_str(s: &str) -> Result<Self, UnsupportedDidMethodError> {
        match s {
            "jwk" => Ok(DidMethod::Jwk),
            "key" => Ok(DidMethod::Key),
            _ => Err(UnsupportedDidMethodError),
        }
    }
}
