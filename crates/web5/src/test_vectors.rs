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
        fn resolve() {
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

    mod did_dht {
        use super::*;
        use crate::dids::{
            methods::did_dht::DidDht,
            resolution::resolution_metadata::{ResolutionMetadata, ResolutionMetadataError},
        };

        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct VectorInput {
            #[serde(rename = "didUri")]
            did_uri: String,
        }

        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct VectorOutput {
            #[serde(rename = "didResolutionMetadata")]
            did_resolution_metadata: ResolutionMetadata,
        }

        #[test]
        fn resolve() {
            let path = "did_dht/resolve.json";
            let vectors: TestVectorFile<VectorInput, VectorOutput> =
                TestVectorFile::load_from_path(path);

            for vector in vectors.vectors {
                let vector_input = vector.input;
                let vector_output = &vector.output;

                // As a replay attack protection protocol, if the same DID is doing a GET request within 5 minutes of each other, instead of a 404 it will start returning a 429.
                // to get around this for our test we just create a new DID that is not published to get a fresh 404 for this error code
                if let Some(ResolutionMetadataError::NotFound) =
                    vector_output.did_resolution_metadata.error
                {
                    // TODO: According to the did dht spec a 404 should be returned when trying to resolve a DID that does not exists. Currently it incorrectly returns a 429 even on the first call.
                    // Uncomment this code block when resolved - https://github.com/TBD54566975/web5-rs/issues/286
                    continue;

                    // let private_jwk = Ed25519Generator::generate();
                    // let identity_key = ed25519::to_public_jwk(&private_jwk);
                    // let did_dht =
                    //     DidDht::from_identity_key(identity_key.clone()).expect("Should create did:dht");
                    //
                    // vector_input = VectorInput{
                    //     did_uri: did_dht.did.uri,
                    // };
                }

                let resolution_result = DidDht::resolve(&vector_input.did_uri);

                let metadata_error = resolution_result.resolution_metadata.error.as_ref();
                let expected_error = vector_output.did_resolution_metadata.error.as_ref();

                assert_eq!(
                    metadata_error, expected_error,
                    "Document resolution metadata does not match. Expected '{:?}' but got '{:?}'.",
                    expected_error, metadata_error
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
        fn select_credentials() {
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
