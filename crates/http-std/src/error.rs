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
}

pub type Result<T> = std::result::Result<T, Error>;
