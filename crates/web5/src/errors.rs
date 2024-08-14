use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Web5Error {
    #[error("json error {0}")]
    Json(String),
}

impl From<SerdeJsonError> for Web5Error {
    fn from(err: SerdeJsonError) -> Self {
        Web5Error::Json(err.to_string())
    }
}
