use crate::dids::{
    data_model::document::Document,
    did::Did,
    resolution::{
        resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
    },
};
use url::Url;

// PORT_SEP is the : character that separates the domain from the port in a URI.
const PORT_SEP: &str = "%3A";

/// Resolver is the implementation of the did:web method for resolving DID URIs. It is responsible
/// for fetching the DID Document from the web according for the did-web spec.
pub struct Resolver {
    http_url: String,
}

impl Resolver {
    pub fn new(did: Did) -> Result<Self, ResolutionMetadataError> {
        // note: delimited is : generally, but ; is allowed by the spec. The did-web spec (§3.2) says
        // ; should be avoided because of it's potential use for matrix URIs.
        let http_url = match did.id.split_once(':') {
            Some((domain, path)) => format!(
                "{}/{}",
                domain.replace(PORT_SEP, ":"),
                path.split(':').collect::<Vec<&str>>().join("/"),
            ),
            None => format!("{}/{}", did.id.replace(PORT_SEP, ":"), ".well-known",),
        };

        let url = Url::parse(&format!("http://{}", http_url))
            .map_err(|_| ResolutionMetadataError::InvalidDid)?;
        let protocol =
            match url.host_str() == Some("localhost") || url.host_str() == Some("127.0.0.1") {
                true => "http",
                false => "https",
            };

        Ok(Self {
            http_url: format!("{}://{}/did.json", protocol, http_url),
        })
    }

    pub async fn resolve(&self) -> Result<ResolutionResult, ResolutionMetadataError> {
        let response = http_std::fetch(&self.http_url, None)
            .await
            .map_err(|_| ResolutionMetadataError::InternalError)?;

        if response.status_code == 404 {
            return Err(ResolutionMetadataError::NotFound);
        } else if !(200..300).contains(&response.status_code) {
            return Err(ResolutionMetadataError::InternalError);
        }

        let document = serde_json::from_slice::<Document>(&response.body)
            .map_err(|_| ResolutionMetadataError::InternalError)?;

        Ok(ResolutionResult {
            document: Some(document),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_success() {
        let did_uri = "did:web:tbd.website";
        let result = Resolver::new(Did::parse(did_uri).unwrap()).unwrap();
        assert_eq!(result.http_url, "https://tbd.website/.well-known/did.json");

        let did_uri = "did:web:tbd.website:with:path";
        let result = Resolver::new(Did::parse(did_uri).unwrap()).unwrap();
        assert_eq!(result.http_url, "https://tbd.website/with/path/did.json");

        let did_uri = "did:web:tbd.website%3A8080";
        let result = Resolver::new(Did::parse(did_uri).unwrap()).unwrap();
        assert_eq!(
            result.http_url,
            "https://tbd.website:8080/.well-known/did.json"
        );

        let did_uri = "did:web:tbd.website%3A8080:with:path";
        let result = Resolver::new(Did::parse(did_uri).unwrap()).unwrap();
        assert_eq!(
            result.http_url,
            "https://tbd.website:8080/with/path/did.json"
        );
    }
}
