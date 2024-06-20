use std::sync::Arc;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::apid::credentials::presentation_definition::PexError;
use web5::apid::credentials::CredentialError;
use web5::apid::crypto::{jwk::JwkError, key_managers::KeyManagerError};
use web5::apid::dids::bearer_did::BearerDidError;
use web5::apid::dids::data_model::DataModelError as DidDataModelError;
use web5::apid::dids::did::DidError;
use web5::apid::dids::methods::MethodError;
use web5::apid::dsa::DsaError;

#[derive(Debug, Error)]
pub enum RustCoreError {
    #[error("{message}")]
    Error {
        r#type: String,
        variant: String,
        message: String,
    },
}

impl RustCoreError {
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            r#type: type_of(&error).to_string(),
            variant: variant_name(&error),
            message: error.to_string(),
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            RustCoreError::Error {
                r#type: error_type, ..
            } => error_type.clone(),
        }
    }

    pub fn variant(&self) -> String {
        match self {
            RustCoreError::Error {
                variant: error_variant,
                ..
            } => error_variant.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            RustCoreError::Error { message, .. } => message.clone(),
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

impl From<JwkError> for RustCoreError {
    fn from(error: JwkError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<KeyManagerError> for RustCoreError {
    fn from(error: KeyManagerError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<DsaError> for RustCoreError {
    fn from(error: DsaError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<DidError> for RustCoreError {
    fn from(error: DidError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<MethodError> for RustCoreError {
    fn from(error: MethodError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<CredentialError> for RustCoreError {
    fn from(error: CredentialError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<PexError> for RustCoreError {
    fn from(error: PexError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<DidDataModelError> for RustCoreError {
    fn from(error: DidDataModelError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<BearerDidError> for RustCoreError {
    fn from(error: BearerDidError) -> Self {
        RustCoreError::new(error)
    }
}

pub type Result<T> = std::result::Result<T, Arc<RustCoreError>>;
