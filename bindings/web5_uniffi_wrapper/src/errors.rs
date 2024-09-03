use serde_json::Error as SerdeJsonError;
use std::fmt::Debug;
use thiserror::Error;
use web5::credentials::presentation_definition::PexError;
use web5::credentials::CredentialError;
use web5::errors::Web5Error as InnerWeb5Error;

#[derive(Debug, Error)]
pub enum Web5Error {
    #[error("{msg}")]
    Error { variant: String, msg: String },
}

impl Web5Error {
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            variant: variant_name(&error),
            msg: error.to_string(),
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

fn variant_name<T>(error: &T) -> String
where
    T: Debug,
{
    let msg = format!("{:?}", error);
    let variant_name = msg.split('(').next().unwrap_or("UnknownVariant");
    variant_name.to_string()
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

impl From<SerdeJsonError> for Web5Error {
    fn from(error: SerdeJsonError) -> Self {
        Web5Error::new(error)
    }
}

impl From<Web5Error> for InnerWeb5Error {
    fn from(error: Web5Error) -> Self {
        let variant = error.variant();
        let msg = error.msg();

        match variant.as_str() {
            "Json" => InnerWeb5Error::Json(msg),
            "JsonSchema" => InnerWeb5Error::JsonSchema(msg),
            "Parameter" => InnerWeb5Error::Parameter(msg),
            "DataMember" => InnerWeb5Error::DataMember(msg),
            "NotFound" => InnerWeb5Error::NotFound(msg),
            "Crypto" => InnerWeb5Error::Crypto(msg),
            "Encoding" => InnerWeb5Error::Encoding(msg),
            "Mutex" => InnerWeb5Error::Mutex(msg),
            _ => InnerWeb5Error::Unknown(format!("unknown variant {} with msg {}", variant, msg)),
        }
    }
}

impl From<InnerWeb5Error> for Web5Error {
    fn from(error: InnerWeb5Error) -> Self {
        Web5Error::new(error)
    }
}

pub type Result<T> = std::result::Result<T, Web5Error>;
