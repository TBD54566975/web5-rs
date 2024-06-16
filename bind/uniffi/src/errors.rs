use std::sync::Arc;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::apid::credentials::CredentialError;
use web5::apid::dids::did::DidError;
use web5::apid::dids::methods::MethodError;
use web5::apid::dsa::DsaError;
use web5::apid::{in_memory_key_manager::KeyManagerError, jwk::JwkError};

#[derive(Debug, Error)]
pub enum RcbError {
    #[error("{message}")]
    Error {
        message: String,
        error_type: String,
        error_variant: String,
    },
}

impl RcbError {
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
            RcbError::Error { message, .. } => message.clone(),
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            RcbError::Error { error_type, .. } => error_type.clone(),
        }
    }

    pub fn error_variant(&self) -> String {
        match self {
            RcbError::Error { error_variant, .. } => error_variant.clone(),
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

impl From<JwkError> for RcbError {
    fn from(error: JwkError) -> Self {
        RcbError::new(error)
    }
}

impl From<KeyManagerError> for RcbError {
    fn from(error: KeyManagerError) -> Self {
        RcbError::new(error)
    }
}

impl From<DsaError> for RcbError {
    fn from(error: DsaError) -> Self {
        RcbError::new(error)
    }
}

impl From<DidError> for RcbError {
    fn from(error: DidError) -> Self {
        RcbError::new(error)
    }
}

impl From<MethodError> for RcbError {
    fn from(error: MethodError) -> Self {
        RcbError::new(error)
    }
}

impl From<CredentialError> for RcbError {
    fn from(error: CredentialError) -> Self {
        RcbError::new(error)
    }
}

pub type RcbResult<T> = std::result::Result<T, Arc<RcbError>>;
