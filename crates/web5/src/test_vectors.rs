use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
pub struct TestVector<I, O> {
    pub description: String,
    pub input: I,
    pub output: O,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestVectorFile<I, O> {
    pub vectors: Vec<TestVector<I, O>>,
}

impl<I, O> TestVectorFile<I, O> {
    pub fn load_from_path(file_path: &str) -> TestVectorFile<I, O>
    where
        I: DeserializeOwned,
        O: DeserializeOwned,
    {
        let mut vector_path = PathBuf::from("../../web5-spec/test-vectors/");
        vector_path.push(file_path);
        let data = fs::read_to_string(vector_path).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

#[cfg(test)]
mod test_vectors {
    use super::*;

    mod did_jwk {
        use super::*;
        use crate::dids::{
            data_model::document::Document,
            methods::did_jwk::DidJwk,
            resolution::{
                document_metadata::DocumentMetadata, resolution_metadata::ResolutionMetadata,
            },
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
        fn test_did_jwk_resolve() {
            let path = "did_jwk/resolve.json";
            let vectors: TestVectorFile<String, VectorOutput> =
                TestVectorFile::load_from_path(path);

            for vector in vectors.vectors {
                let did_uri = vector.input;
                let resolution_result = DidJwk::resolve(&did_uri);

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

    mod presentation_definition {
        use super::*;
        use crate::credentials::presentation_definition::PresentationDefinition;
        use std::collections::HashSet;

        #[derive(Debug, serde::Deserialize)]
        struct VectorInput {
            #[serde(rename = "presentationDefinition")]
            pub presentation_definition: PresentationDefinition,
            #[serde(rename = "credentialJwts")]
            pub credential_jwts: Vec<String>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct VectorOutput {
            #[serde(rename = "selectedCredentials")]
            pub selected_credentials: Vec<String>,
        }

        #[test]
        #[ignore] // TODO temporarily ignoring, because web5-spec test vectors use did:key which isn't supported
        fn test_presentation_exchange_select_credentials() {
            let path = "presentation_exchange/select_credentials.json";
            let vectors: TestVectorFile<VectorInput, VectorOutput> =
                TestVectorFile::load_from_path(path);

            for vector in vectors.vectors {
                let presentation_definition = vector.input.presentation_definition;
                let vc_jwts = vector.input.credential_jwts;
                let error_msg = format!(
                    "Selected Credential test vector ({}) should not have thrown error",
                    vector.description
                );

                let selected_credentials = presentation_definition
                    .select_credentials(&vc_jwts)
                    .expect(&error_msg);

                let set1: HashSet<_> = selected_credentials.iter().collect();
                let set2: HashSet<_> = vector.output.selected_credentials.iter().collect();
                assert_eq!(
                    set1, set2,
                    "Vectors do not contain the same elements: {}",
                    error_msg
                );
            }
        }
    }
}
