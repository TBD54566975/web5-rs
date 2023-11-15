use crate::did::method::{DidMethod, UnsupportedDidMethodError}; // TODO: move this to mod?
use std::str::FromStr;

pub struct Did {
    pub uri: String,
    pub method: DidMethod,
    pub method_id: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DidError {
    #[error("Provided Did URI is invalid")]
    InvalidDidUri,
    #[error(transparent)]
    ParseError(#[from] UnsupportedDidMethodError),
}

impl FromStr for Did {
    type Err = DidError;

    fn from_str(did_uri: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = did_uri.splitn(3, ':').collect();
        if parts.len() == 3 && parts[0] == "did" {
            Ok(Self {
                uri: did_uri.to_string(),
                method: DidMethod::from_str(parts[1])?,
                method_id: parts[2].to_string(),
            })
        } else {
            Err(DidError::InvalidDidUri)
        }
    }
}
