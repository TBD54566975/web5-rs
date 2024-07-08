use super::{MethodError, Result};
use crate::{
    crypto::jwk::Jwk,
    dids::{
        data_model::{document::Document, verification_method::VerificationMethod},
        did::Did,
        resolution::{
            resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
            resolution_result::ResolutionResult,
        },
    },
};
use base64::{engine::general_purpose, Engine as _};

#[derive(Clone)]
pub struct DidJwk {
    pub did: Did,
    pub document: Document,
}

impl DidJwk {
    pub fn from_public_jwk(public_jwk: Jwk) -> Result<Self> {
        let jwk_string = serde_json::to_string(&public_jwk)?;
        let method_specific_id = general_purpose::URL_SAFE_NO_PAD.encode(jwk_string);

        let uri = format!("did:jwk:{}", method_specific_id);

        let did = Did::new(&uri)?;

        let verification_method_id = format!("{}#0", uri);

        let verification_method = VerificationMethod {
            id: verification_method_id.clone(),
            r#type: "JsonWebKey".to_string(),
            controller: uri.clone(),
            public_key_jwk: public_jwk.clone(),
        };

        let document = Document {
            id: uri.clone(),
            verification_method: vec![verification_method.clone()],
            authentication: Some(vec![verification_method_id.clone()]),
            assertion_method: Some(vec![verification_method_id.clone()]),
            capability_invocation: Some(vec![verification_method_id.clone()]),
            capability_delegation: Some(vec![verification_method_id.clone()]),
            ..Default::default()
        };

        Ok(Self { did, document })
    }

    pub fn from_uri(uri: &str) -> Result<Self> {
        let resolution_result = DidJwk::resolve(uri);

        match resolution_result.document {
            None => Err(match resolution_result.resolution_metadata.error {
                None => MethodError::ResolutionError(ResolutionMetadataError::InternalError),
                Some(e) => MethodError::ResolutionError(e),
            }),
            Some(document) => {
                let did = Did::new(uri)?;
                Ok(Self { did, document })
            }
        }
    }

    pub fn resolve(uri: &str) -> ResolutionResult {
        let result: Result<ResolutionResult> = (|| {
            let did = Did::new(uri).map_err(|_| ResolutionMetadataError::InvalidDid)?;
            let decoded_jwk = general_purpose::URL_SAFE_NO_PAD
                .decode(did.id)
                .map_err(|_| ResolutionMetadataError::InvalidDid)?;
            let public_jwk = serde_json::from_slice::<Jwk>(&decoded_jwk)
                .map_err(|_| ResolutionMetadataError::InvalidDid)?;

            let kid = format!("{}#0", did.uri);
            let document = Document {
                context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
                id: did.uri.clone(),
                verification_method: vec![VerificationMethod {
                    id: kid.clone(),
                    r#type: "JsonWebKey".to_string(),
                    controller: did.uri.clone(),
                    public_key_jwk: public_jwk,
                }],
                assertion_method: Some(vec![kid.clone()]),
                authentication: Some(vec![kid.clone()]),
                capability_invocation: Some(vec![kid.clone()]),
                capability_delegation: Some(vec![kid.clone()]),

                // TODO: https://github.com/TBD54566975/web5-rs/issues/257 - If the JWK contains a `use` property with the value "sig" then the `keyAgreement` property
                // is not included in the DID Document. If the `use` value is "enc" then only the `keyAgreement`
                // property is included in the DID Document.
                // key_agreement: if public_jwk.use_.as_deref() != Some("sig") { Some(vec![kid.clone()]) } else { None },
                ..Default::default()
            };

            Ok(ResolutionResult {
                document: Some(document),
                ..Default::default()
            })
        })();

        match result {
            Ok(resolution_result) => resolution_result,
            Err(err) => ResolutionResult {
                resolution_metadata: ResolutionMetadata {
                    error: Some(match err {
                        MethodError::ResolutionError(e) => e,
                        _ => ResolutionMetadataError::InternalError,
                    }),
                },
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dids::{
            data_model::document::Document,
            resolution::{
                document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata,
            },
        },
        test_helpers::TestVectorFile,
    };

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct VectorOutput {
        #[serde(rename = "@context")]
        context: String,
        #[serde(rename = "didDocument")]
        did_document: Option<Document>,
        #[serde(rename = "didDocumentMetadata")]
        did_document_metadata: DocumentMetadata,
        #[serde(rename = "didResolutionMetadata")]
        did_resolution_metadata: ResolutionMetadata,
    }

    #[test]
    fn test_web5_spec_did_jwk_test_vectors() {
        let path = "did_jwk/resolve.json";
        let vectors: TestVectorFile<String, VectorOutput> = TestVectorFile::load_from_path(path);

        for vector in vectors.vectors {
            let did_uri = vector.input;
            let resolution_result = super::DidJwk::resolve(&did_uri);

            let all_none = vector.output.did_document_metadata.created.is_none()
                && vector.output.did_document_metadata.updated.is_none()
                && vector.output.did_document_metadata.deactivated.is_none()
                && vector.output.did_document_metadata.next_update.is_none()
                && vector.output.did_document_metadata.version_id.is_none()
                && vector
                    .output
                    .did_document_metadata
                    .next_version_id
                    .is_none()
                && vector.output.did_document_metadata.equivalent_id.is_none()
                && vector.output.did_document_metadata.canonical_id.is_none();

            let vector_document_metadata = if all_none {
                None
            } else {
                Some(vector.output.did_document_metadata.clone())
            };

            assert_eq!(
                resolution_result.resolution_metadata, vector.output.did_resolution_metadata,
                "Resolution metadata does not match."
            );
            assert_eq!(
                resolution_result.document, vector.output.did_document,
                "DID Document does not match."
            );
            assert_eq!(
                resolution_result.document_metadata, vector_document_metadata,
                "Document metadata does not match."
            );
        }
    }
}
