use crate::did_jwk::DidJwk;
use crate::did_key::DidKey;
use crate::error::Web5Error;
use async_trait::async_trait;

pub trait Did {
    fn uri(&self) -> String;
}

#[derive(uniffi::Record)]
pub struct DidResolutionResult {
    pub did_document: String,
    pub did_document_metadata: Option<String>,
}

#[async_trait]
pub(crate) trait DidResolver {
    fn method_name() -> &'static str;
    async fn resolve(did_uri: &str) -> Result<DidResolutionResult, Web5Error>;
}

#[uniffi::export]
async fn resolve(did_uri: String) -> Result<DidResolutionResult, Web5Error> {
    let did_method = parse_did_method(&did_uri)?;

    match did_method.as_str() {
        "jwk" => DidJwk::resolve(&did_uri).await,
        "key" => DidKey::resolve(&did_uri).await,
        _ => Err(Web5Error::Unknown),
    }
}

fn parse_did_method(did: &str) -> Result<String, Web5Error> {
    let parts: Vec<&str> = did.splitn(3, ':').collect();
    if parts.len() == 3 && parts[0] == "did" {
        return Ok(parts[1].to_string());
    }
    Err(Web5Error::Unknown)
}
