use crate::crypto::key_manager::KeyManagerError;

#[derive(thiserror::Error, Debug)]
pub enum Web5Error {
    #[error("internal web5 error: {message}")]
    InternalError { message: String },
}

impl From<KeyManagerError> for Web5Error {
    fn from(error: KeyManagerError) -> Self {
        Self::InternalError {
            message: error.to_string(),
        }
    }
}
