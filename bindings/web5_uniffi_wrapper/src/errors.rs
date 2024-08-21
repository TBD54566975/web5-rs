use serde_json::Error as SerdeJsonError;
use std::sync::PoisonError;
use std::{any::type_name, fmt::Debug};
use thiserror::Error;
use web5::credentials::presentation_definition::PexError;
use web5::credentials::CredentialError;
use web5::crypto::key_managers::KeyManagerError;
use web5::dids::bearer_did::BearerDidError;
use web5::dids::methods::MethodError;
use web5::errors::Web5Error as InnerWeb5Error;

#[derive(Debug, Error)]
pub enum Web5Error {
    #[error("{msg}")]
    Error {
        r#type: String,
        variant: String,
        msg: String,
    },
}

impl Web5Error {
    pub fn from_poison_error<T>(error: PoisonError<T>, error_type: &str) -> Self {
        Web5Error::Error {
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
            Web5Error::Error {
                r#type: error_type, ..
            } => error_type.clone(),
        }
    }

    pub fn variant(&self) -> String {
        match self {
            Web5Error::Error {
                variant: error_variant,
                ..
            } => error_variant.clone(),
        }
    }

    pub fn msg(&self) -> String {
        match self {
            Web5Error::Error { msg, .. } => msg.clone(),
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

impl From<KeyManagerError> for Web5Error {
    fn from(error: KeyManagerError) -> Self {
        Web5Error::new(error)
    }
}

impl From<MethodError> for Web5Error {
    fn from(error: MethodError) -> Self {
        Web5Error::new(error)
    }
}

impl From<CredentialError> for Web5Error {
    fn from(error: CredentialError) -> Self {
        Web5Error::new(error)
    }
}

impl From<PexError> for Web5Error {
    fn from(error: PexError) -> Self {
        Web5Error::new(error)
    }
}

impl From<BearerDidError> for Web5Error {
    fn from(error: BearerDidError) -> Self {
        Web5Error::new(error)
    }
}

impl From<SerdeJsonError> for Web5Error {
    fn from(error: SerdeJsonError) -> Self {
        Web5Error::new(error)
    }
}

impl From<Web5Error> for KeyManagerError {
    fn from(error: Web5Error) -> Self {
        let variant = error.variant();
        let msg = error.msg();

        if variant == variant_name(&KeyManagerError::KeyGenerationFailed) {
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

impl From<Web5Error> for InnerWeb5Error {
    fn from(error: Web5Error) -> Self {
        let variant = error.variant();
        let msg = error.msg();

        if variant == variant_name(&InnerWeb5Error::Json(String::default())) {
            return InnerWeb5Error::Json(msg);
        } else if variant == variant_name(&InnerWeb5Error::Parameter(String::default())) {
            return InnerWeb5Error::Parameter(msg);
        } else if variant == variant_name(&InnerWeb5Error::DataMember(String::default())) {
            return InnerWeb5Error::DataMember(msg);
        } else if variant == variant_name(&InnerWeb5Error::NotFound(String::default())) {
            return InnerWeb5Error::NotFound(msg);
        } else if variant == variant_name(&InnerWeb5Error::Crypto(String::default())) {
            return InnerWeb5Error::Crypto(msg);
        } else if variant == variant_name(&InnerWeb5Error::Encoding(String::default())) {
            return InnerWeb5Error::Encoding(msg);
        }

        InnerWeb5Error::Unknown(format!("unknown variant {} with msg {}", variant, msg))
    }
}

impl From<InnerWeb5Error> for Web5Error {
    fn from(error: InnerWeb5Error) -> Self {
        Web5Error::new(error)
    }
}

pub type Result<T> = std::result::Result<T, Web5Error>;
