use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use reqwest::header::{HeaderMap, HeaderValue};

use crate::dids::{
    document::Document,
    identifier::Identifier,
    resolver::{ResolutionError, ResolutionResult},
};

// PORT_SEP is the : character that separates the domain from the port in a URI.
const PORT_SEP: &str = "%3A";

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Resolver is the implementation of the did:web method for resolcing DID URIs. It is responsible
/// for fetching the DID Document from the web according for the did-web spec.
pub struct Resolver {
    did_url: String,
}

impl Resolver {
    pub fn new(did_uri: Identifier) -> Self {
        // note: delimited is : generally, but ; is allowed by the spec. The did-web spec (§3.2) says
        // ; should be avoided because of it's potential use for matrix URIs.
        let did_url = match did_uri.id.split_once(':') {
            Some((domain, path)) => format!(
                "{}/{}",
                domain.replace(PORT_SEP, ":"),
                path.split(':').collect::<Vec<&str>>().join("/"),
            ),
            None => format!("{}/{}", did_uri.id.replace(PORT_SEP, ":"), ".well-known",),
        };

        Self {
            did_url: format!("https://{}/did.json", did_url),
        }
    }
}

// This trait implements the actual logic for resolving a DID URI to a DID Document.
impl IntoFuture for Resolver {
    type Output = Result<ResolutionResult, ResolutionError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

    fn into_future(self) -> Self::IntoFuture {
        let mut headers = HeaderMap::new();
        headers.append(
            reqwest::header::USER_AGENT,
            HeaderValue::from_static(USER_AGENT),
        );

        Box::pin(async move {
            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| ResolutionError::InternalError)?;

            let response = client
                .get(&self.did_url)
                .send()
                .await
                .map_err(|_| ResolutionError::InternalError)?;

            if response.status().is_success() {
                let did_document = response
                    .json::<Document>()
                    .await
                    .map_err(|_| ResolutionError::RepresentationNotSupported)?;

                Ok(ResolutionResult {
                    did_document: Some(did_document),
                    ..Default::default()
                })
            } else {
                Err(ResolutionError::NotFound)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn resolution_success() {
        let did_uri = "did:web:tbd.website";
        let result = Resolver::new(Identifier::parse(did_uri).unwrap());
        assert_eq!(result.did_url, "https://tbd.website/.well-known/did.json");

        let did_uri = "did:web:tbd.website:with:path";
        let result = Resolver::new(Identifier::parse(did_uri).unwrap());
        assert_eq!(result.did_url, "https://tbd.website/with/path/did.json");
    }
}
