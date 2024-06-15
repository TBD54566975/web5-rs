use std::sync::Arc;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::apid::{in_memory_key_manager::KeyManagerError, jwk::JwkError};

#[derive(Debug, Error)]
pub enum UniffiWeb5Error {
    #[error("{message}")]
    Error {
        message: String,
        error_type: String,
        error_variant: String,
    },
}

impl UniffiWeb5Error {
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            message: error.to_string(),
            error_type: type_of(&error).to_string(),
            error_variant: variant_name(&error),
        }
    }

    pub fn message(&self) -> String {
        match self {
            UniffiWeb5Error::Error { message, .. } => message.clone(),
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            UniffiWeb5Error::Error { error_type, .. } => error_type.clone(),
        }
    }

    pub fn error_variant(&self) -> String {
        match self {
            UniffiWeb5Error::Error { error_variant, .. } => error_variant.clone(),
        }
    }
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn variant_name<T>(error: &T) -> String
where
    T: Debug,
{
    let message = format!("{:?}", error);
    let variant_name = message.split('(').next().unwrap_or("UnknownVariant");
    variant_name.to_string()
}

impl From<JwkError> for UniffiWeb5Error {
    fn from(error: JwkError) -> Self {
        UniffiWeb5Error::new(error)
    }
}

impl From<KeyManagerError> for UniffiWeb5Error {
    fn from(error: KeyManagerError) -> Self {
        UniffiWeb5Error::new(error)
    }
}

pub fn test_jwk_err() -> Result<(), Arc<UniffiWeb5Error>> {
    Err(Arc::new(
        JwkError::ThumbprintFailed("testing inner string".to_string()).into(),
    ))
}

pub fn test_key_manager_err() -> Result<(), Arc<UniffiWeb5Error>> {
    Err(Arc::new(
        KeyManagerError::KeyNotFound("test_key".to_string()).into(),
    ))
}
