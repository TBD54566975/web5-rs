use crate::did_jwk::DidJwk;
use crate::error::Web5Error;
use async_trait::async_trait;
use did_jwk::DIDJWK;

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
    DidJwk::resolve(&did_uri).await
}
