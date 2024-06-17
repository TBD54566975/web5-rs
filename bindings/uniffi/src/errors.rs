use std::sync::Arc;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::apid::credentials::presentation_definition::PexError;
use web5::apid::credentials::CredentialError;
use web5::apid::dids::did::DidError;
use web5::apid::dids::methods::MethodError;
use web5::apid::dsa::DsaError;
use web5::apid::{in_memory_key_manager::KeyManagerError, jwk::JwkError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("{message}")]
    Error {
        message: String,
        error_type: String,
        error_variant: String,
    },
}

impl Error {
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
            Error::Error { message, .. } => message.clone(),
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            Error::Error { error_type, .. } => error_type.clone(),
        }
    }

    pub fn error_variant(&self) -> String {
        match self {
            Error::Error { error_variant, .. } => error_variant.clone(),
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

impl From<JwkError> for Error {
    fn from(error: JwkError) -> Self {
        Error::new(error)
    }
}

impl From<KeyManagerError> for Error {
    fn from(error: KeyManagerError) -> Self {
        Error::new(error)
    }
}

impl From<DsaError> for Error {
    fn from(error: DsaError) -> Self {
        Error::new(error)
    }
}

impl From<DidError> for Error {
    fn from(error: DidError) -> Self {
        Error::new(error)
    }
}

impl From<MethodError> for Error {
    fn from(error: MethodError) -> Self {
        Error::new(error)
    }
}

impl From<CredentialError> for Error {
    fn from(error: CredentialError) -> Self {
        Error::new(error)
    }
}

impl From<PexError> for Error {
    fn from(error: PexError) -> Self {
        Error::new(error)
    }
}

pub type Result<T> = std::result::Result<T, Arc<Error>>;
