mod resolver;

use crate::dids::{
    identifier::Identifier,
    methods::{Method, ResolutionResult, Resolve},
    resolver::ResolutionError,
};
use resolver::Resolver;

/// Concrete implementation for a did:web DID
pub struct DidWeb {}

impl Method for DidWeb {
    const NAME: &'static str = "web";
}

impl Resolve for DidWeb {
    async fn resolve(did_uri: &str) -> ResolutionResult {
        let identifier = match Identifier::parse(did_uri) {
            Ok(identifier) => identifier,
            Err(_) => return ResolutionResult::from_error(ResolutionError::InvalidDid),
        };

        match Resolver::new(identifier).await {
            Ok(result) => result,
            Err(e) => ResolutionResult::from_error(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn resolution_success() {
        let did_uri = "did:web:tbd.website";
        let result = DidWeb::resolve(did_uri).await;
        assert!(result.did_resolution_metadata.error.is_none());

        let did_document = result.did_document.expect("did_document not found");
        assert_eq!(did_document.id, did_uri);
    }

    #[tokio::test]
    async fn resolution_failure() {
        let result = DidWeb::resolve("did:web:does-not-exist").await;
        assert!(result.did_resolution_metadata.error.is_some());
    }
}
