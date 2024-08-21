use base64::DecodeError;
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Web5Error {
    #[error("unknown error {0}")]
    Unknown(String),
    #[error("json error {0}")]
    Json(String),
    #[error("parameter error {0}")]
    Parameter(String),
    #[error("data member error {0}")]
    DataMember(String),
    #[error("not found error {0}")]
    NotFound(String),
    #[error("cryptography error {0}")]
    Crypto(String),
    #[error("encoding error {0}")]
    Encoding(String),
}

impl From<DecodeError> for Web5Error {
    fn from(err: DecodeError) -> Self {
        Web5Error::Encoding(err.to_string())
    }
}

impl From<SerdeJsonError> for Web5Error {
    fn from(err: SerdeJsonError) -> Self {
        Web5Error::Json(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Web5Error>;
