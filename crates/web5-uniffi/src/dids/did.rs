use crate::did_jwk::DidJwkResolver;
use crate::did_key::DidKeyResolver;
use crate::{Web5Error, Web5Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub trait Did {
    fn uri(&self) -> String;
}

#[derive(uniffi::Record)]
pub struct DidResolutionResult {
    pub did_document: String,
    pub did_document_metadata: Option<String>,
}

#[async_trait]
pub(crate) trait DidResolver: Sync + Send {
    fn method_name() -> &'static str
    where
        Self: Sized;
    async fn resolve(&self, did_uri: &str) -> Web5Result<DidResolutionResult>;
}

type BoxedResolver = Arc<dyn DidResolver>;

fn create_resolvers_map() -> HashMap<&'static str, BoxedResolver> {
    let mut resolvers: HashMap<&'static str, BoxedResolver> = HashMap::new();
    resolvers.insert(DidJwkResolver::method_name(), Arc::new(DidJwkResolver));
    resolvers.insert(DidKeyResolver::method_name(), Arc::new(DidKeyResolver));
    resolvers
}

#[uniffi::export]
async fn resolve(did_uri: String) -> Web5Result<DidResolutionResult> {
    let did_method = parse_did_method(&did_uri)?;
    let resolvers = create_resolvers_map();

    // TODO: amika - Address this
    if let Some(resolver) = resolvers.get(did_method.as_str()) {
        resolver.resolve(&did_uri).await
    } else {
        Err(Web5Error::Unknown)
    }
}

fn parse_did_method(did: &str) -> Web5Result<String> {
    let parts: Vec<&str> = did.splitn(3, ':').collect();
    if parts.len() == 3 && parts[0] == "did" {
        return Ok(parts[1].to_string());
    }
    Err(Web5Error::Unknown)
}
