pub mod presentation_definition;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PexError {
    #[error("Failed to parse JSON: {0}")]
    JsonError(String),
}

type Result<T> = std::result::Result<T, PexError>;
