use serde_json::Error as SerdeJsonError;
use std::sync::PoisonError;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::credentials::presentation_definition::PexError;
use web5::credentials::CredentialError;
use web5::crypto::dsa::DsaError;
use web5::crypto::{jwk::JwkError, key_managers::KeyManagerError};
use web5::dids::bearer_did::BearerDidError;
use web5::dids::data_model::DataModelError as DidDataModelError;
use web5::dids::did::DidError;
use web5::dids::methods::MethodError;
use web5::dids::portable_did::PortableDidError;

#[derive(Debug, Error)]
pub enum RustCoreError {
    #[error("{msg}")]
    Error {
        r#type: String,
        variant: String,
        msg: String,
    },
}

impl RustCoreError {
    pub fn from_poison_error<T>(error: PoisonError<T>, error_type: &str) -> Self {
        RustCoreError::Error {
            r#type: error_type.to_string(),
            variant: "PoisonError".to_string(),
            msg: error.to_string(),
        }
    }

    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            r#type: type_of(&error).to_string(),
            variant: variant_name(&error),
            msg: error.to_string(),
        }
    }

    pub fn r#type(&self) -> String {
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

    pub fn msg(&self) -> String {
        match self {
            RustCoreError::Error { msg, .. } => msg.clone(),
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
    let msg = format!("{:?}", error);
    let variant_name = msg.split('(').next().unwrap_or("UnknownVariant");
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

impl From<PortableDidError> for RustCoreError {
    fn from(error: PortableDidError) -> Self {
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

impl From<SerdeJsonError> for RustCoreError {
    fn from(error: SerdeJsonError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<RustCoreError> for KeyManagerError {
    fn from(error: RustCoreError) -> Self {
        let variant = error.variant();
        let msg = error.msg();

        if variant
            == variant_name(&KeyManagerError::JwkError(JwkError::ThumbprintFailed(
                String::default(),
            )))
        {
            return KeyManagerError::JwkError(JwkError::ThumbprintFailed(msg.to_string()));
        } else if variant == variant_name(&KeyManagerError::KeyGenerationFailed) {
            return KeyManagerError::KeyGenerationFailed;
        } else if variant
            == variant_name(&KeyManagerError::InternalKeyStoreError(String::default()))
        {
            return KeyManagerError::InternalKeyStoreError(msg.to_string());
        } else if variant == variant_name(&KeyManagerError::KeyNotFound(String::default())) {
            return KeyManagerError::KeyNotFound(msg.to_string());
        }

        KeyManagerError::Unknown
    }
}

impl From<RustCoreError> for DsaError {
    fn from(error: RustCoreError) -> Self {
        let variant = error.variant();
        let msg = error.msg();

        if variant == variant_name(&DsaError::MissingPrivateKey) {
            return DsaError::MissingPrivateKey;
        } else if variant == variant_name(&DsaError::DecodeError(String::default())) {
            return DsaError::DecodeError(msg);
        } else if variant == variant_name(&DsaError::InvalidKeyLength(String::default())) {
            return DsaError::InvalidKeyLength(msg);
        } else if variant == variant_name(&DsaError::InvalidSignatureLength(String::default())) {
            return DsaError::InvalidSignatureLength(msg);
        } else if variant == variant_name(&DsaError::PublicKeyFailure(String::default())) {
            return DsaError::PublicKeyFailure(msg);
        } else if variant == variant_name(&DsaError::PrivateKeyFailure(String::default())) {
            return DsaError::PrivateKeyFailure(msg);
        } else if variant == variant_name(&DsaError::VerificationFailure(String::default())) {
            return DsaError::VerificationFailure(msg);
        } else if variant == variant_name(&DsaError::SignFailure(String::default())) {
            return DsaError::SignFailure(msg);
        } else if variant == variant_name(&DsaError::UnsupportedDsa) {
            return DsaError::UnsupportedDsa;
        }

        DsaError::Unknown
    }
}

pub type Result<T> = std::result::Result<T, RustCoreError>;
