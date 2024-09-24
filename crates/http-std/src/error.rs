use reqwest::Error as ReqwestError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("unknown error {0}")]
    Unknown(String),
    #[error("parameter error {0}")]
    Parameter(String),
    #[error("network error {0}")]
    Network(String),
    #[error("response error {0}")]
    Response(String),

    #[error("reqwest error {0}")]
    Reqwest(String),
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
