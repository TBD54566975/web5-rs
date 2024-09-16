use super::verifiable_credential_1_1::{VerifiableCredential, VerifiableCredentialCreateOptions};
use crate::credentials::{CredentialSubject, Issuer};
use crate::errors::{Result, Web5Error};
use crate::json::{JsonObject, JsonValue};
use base64::Engine;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::io::{Read, Write};

pub const STATUS_LIST_CREDENTIAL_CONTEXT: &str = "https://w3id.org/vc/status-list/2021/v1";
pub const STATUS_LIST_CREDENTIAL_TYPE: &str = "StatusList2021Credential";
pub const STATUS_LIST_2021: &str = "StatusList2021";

pub const STATUS_LIST_2021_ENTRY: &str = "StatusList2021Entry";

/// Represents a Status List Credential, which is used to maintain the revocation or suspension status of multiple Verifiable Credentials.
/// A Status List Credential is a special type of Verifiable Credential that tracks the status of other credentials.
pub struct StatusListCredential {
    /// The base Verifiable Credential associated with the Status List.
    pub base: VerifiableCredential,
}

impl StatusListCredential {
    /// Creates a new Status List Credential with the specified issuer, status purpose,
    /// and the encoded bitstring list of disabled credentials.
    ///
    /// # Arguments
    ///
    /// * `issuer` - The entity issuing the Status List Credential.
    /// * `status_purpose` - The purpose of the status (e.g., "revocation").
    /// * `disabled_credentials` - A list of Verifiable Credentials that are disabled (revoked or suspended).
    ///
    /// # Example
    /// ```rust
    /// let issuer_bearer_did = DidJwk::create(None).unwrap();
    /// let subject_did_uri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy";
    ///
    /// let verifiable_credential = VerifiableCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     CredentialSubject {
    ///         id: subject_did_uri.to_string(),
    ///         additional_properties: None,
    ///     },
    ///     Some(VerifiableCredentialCreateOptions {
    ///         credential_status: Some(CredentialStatus {
    ///             id: "https://example.com/status/1".to_string(),
    ///             r#type: STATUS_LIST_2021_ENTRY.to_string(),
    ///             status_purpose: "revocation".to_string(),
    ///             status_list_index: "3".to_string(),
    ///             status_list_credential: "https://example.com/status/1".to_string(),
    ///         }),
    ///         ..Default::default()
    ///     }),
    /// ).unwrap();
    ///
    /// let status_list_credential = StatusListCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     "revocation".to_string(),
    ///     Some(vec![verifiable_credential.clone()]),
    /// ).unwrap();
    /// ```
    pub fn create(
        issuer: Issuer,
        status_purpose: String,
        disabled_credentials: Option<Vec<VerifiableCredential>>,
    ) -> Result<Self> {
        // Determine the status list indexes based on the provided credentials to disable.
        let status_list_indexes = match disabled_credentials {
            Some(credentials) => Self::get_status_list_indexes(&status_purpose, credentials)?,
            None => Vec::new(),
        };

        // Generate the base64 bitstring from the status list indexes.
        let base64_bitstring = Self::bitstring_generation(status_list_indexes)?;

        // Construct the properties for the credential subject.
        let additional_properties = JsonObject {
            properties: [
                (
                    "statusPurpose".to_string(),
                    JsonValue::String(status_purpose),
                ),
                (
                    "type".to_string(),
                    JsonValue::String(STATUS_LIST_2021.to_string()),
                ),
                (
                    "encodedList".to_string(),
                    JsonValue::String(base64_bitstring),
                ),
            ]
            .into_iter()
            .collect(),
        };

        let credential_subject = CredentialSubject {
            id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            additional_properties: Some(additional_properties),
        };

        let vc_options = VerifiableCredentialCreateOptions {
            id: Some(format!("urn:uuid:{}", uuid::Uuid::new_v4())),
            context: Some(vec![STATUS_LIST_CREDENTIAL_CONTEXT.to_string()]),
            r#type: Some(vec![STATUS_LIST_CREDENTIAL_TYPE.to_string()]),
            ..Default::default()
        };

        let verifiable_credential =
            VerifiableCredential::create(issuer, credential_subject, Some(vc_options))?;

        Ok(Self {
            base: verifiable_credential,
        })
    }

    /// Checks if a given credential is disabled according to this Status List Credential.
    ///
    /// # Arguments
    ///
    /// * `credential` - The `VerifiableCredential` to check.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the credential is disabled, `Ok(false)` otherwise.
    /// * `Err` if the credential status is invalid or incompatible.
    ///
    /// # Example
    /// ```rust
    /// use web5::credentials::CredentialStatus;
    /// use web5::credentials::CredentialSubject;
    /// use web5::credentials::Issuer;
    /// use web5::credentials::STATUS_LIST_2021_ENTRY;
    /// use web5::credentials::StatusListCredential;
    /// use web5::credentials::VerifiableCredential;
    /// use web5::credentials::VerifiableCredentialCreateOptions;
    /// use web5::dids::methods::did_jwk::DidJwk;
    ///
    /// let issuer_bearer_did = DidJwk::create(None).unwrap();
    /// let subject_did_uri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy";
    ///
    /// let verifiable_credential = VerifiableCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     CredentialSubject {
    ///         id: subject_did_uri.to_string(),
    ///         additional_properties: None,
    ///     },
    ///     Some(VerifiableCredentialCreateOptions {
    ///         credential_status: Some(CredentialStatus {
    ///             id: "https://example.com/status/1".to_string(),
    ///             r#type: STATUS_LIST_2021_ENTRY.to_string(),
    ///             status_purpose: "revocation".to_string(),
    ///             status_list_index: "3".to_string(),
    ///             status_list_credential: "https://example.com/status/1".to_string(),
    ///         }),
    ///         ..Default::default()
    ///     }),
    /// ).unwrap();
    ///
    /// let status_list_credential = StatusListCredential::create(
    ///     Issuer::String(issuer_bearer_did.did.uri.clone()),
    ///     "revocation".to_string(),
    ///     Some(vec![verifiable_credential.clone()]),
    /// ).unwrap();
    ///
    /// let is_disabled = status_list_credential
    ///     .is_disabled(&verifiable_credential)
    ///     .unwrap();
    /// ```
    pub fn is_disabled(&self, credential: &VerifiableCredential) -> Result<bool> {
        let status = credential.credential_status.as_ref().ok_or_else(|| {
            Web5Error::Parameter("no credential status found in credential".to_string())
        })?;

        // Check if the status type matches
        if status.r#type != STATUS_LIST_2021_ENTRY {
            return Err(Web5Error::Parameter(format!(
                "unsupported status type: {}",
                status.r#type
            )));
        }

        // Check if the status purpose matches
        let status_purpose = Self::get_additional_property(
            &self.base.credential_subject.additional_properties,
            "statusPurpose",
        )?;

        if status_purpose != status.status_purpose {
            return Err(Web5Error::Parameter("status purpose mismatch".to_string()));
        }

        // Get the bit index
        let index = status.status_list_index.parse::<usize>().map_err(|_| {
            Web5Error::Parameter(format!(
                "invalid status list index: {}",
                status.status_list_index
            ))
        })?;

        let encoded_list = Self::get_additional_property(
            &self.base.credential_subject.additional_properties,
            "encodedList",
        )?;

        // Check the bit in the encoded list
        Self::get_bit(encoded_list, index)
    }

    /// Extracts status list indexes from a vector of verifiable credentials that match the specified status purpose.
    ///
    /// # Arguments
    /// * `status_purpose` - The status purpose to match.
    /// * `credentials` - A vector of `VerifiableCredential` objects.
    ///
    /// # Returns
    /// A `Result` containing a vector of `usize` indexes, or an error if a credential is missing
    fn get_status_list_indexes(
        status_purpose: &str,
        credentials: Vec<VerifiableCredential>,
    ) -> Result<Vec<usize>> {
        let mut status_list_indexes = Vec::new();

        for vc in credentials {
            let status_list_entry = vc.credential_status.as_ref().ok_or_else(|| {
                Web5Error::Parameter("no credential status found in credential".to_string())
            })?;

            if status_list_entry.status_purpose != *status_purpose {
                return Err(Web5Error::Parameter(format!(
                    "status purpose mismatch: expected '{}', found '{}'",
                    status_purpose, status_list_entry.status_purpose
                )));
            }

            let index = status_list_entry
                .status_list_index
                .parse::<usize>()
                .map_err(|_| {
                    Web5Error::Parameter(format!(
                        "invalid status list index: {}",
                        status_list_entry.status_list_index
                    ))
                })?;

            status_list_indexes.push(index);
        }

        Ok(status_list_indexes)
    }

    /// Generates a compressed, base64-encoded bitstring from a list of status list indexes.
    ///
    /// # Arguments
    /// * `status_list_indexes` - A vector of indexes to set in the bitstring.
    ///
    /// # Returns
    /// A `Result` containing the compressed, base64-encoded bitstring, or an error if an index is out of range.
    fn bitstring_generation(status_list_indexes: Vec<usize>) -> Result<String> {
        const BITSET_SIZE: usize = 16 * 1024 * 8;
        let mut bit_vec = vec![0u8; BITSET_SIZE / 8];

        for index in status_list_indexes {
            if index >= BITSET_SIZE {
                return Err(Web5Error::Parameter(format!(
                    "invalid status list index: {}, index is larger than the bitset size",
                    index
                )));
            }
            let byte_index = index / 8;
            let bit_index = 7 - (index % 8);
            bit_vec[byte_index] |= 1 << bit_index;
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bit_vec).map_err(|e| {
            Web5Error::Parameter(format!(
                "encoder write_all issue while creating bitstring: {}",
                e
            ))
        })?;
        let compressed = encoder.finish().map_err(|e| {
            Web5Error::Parameter(format!(
                "encoder finish issue while creating bitstring: {}",
                e
            ))
        })?;

        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(compressed))
    }

    /// Retrieves the value of a specific bit from a compressed base64 URL-encoded bitstring
    /// by decoding and decompressing a bitstring, then extracting a bit's value by its index.
    ///
    /// # Arguments
    /// * `compressed_bitstring` - A base64 URL-encoded string representing the compressed bitstring.
    /// * `bit_index` - The zero-based index of the bit to retrieve from the decompressed bitstream.
    ///
    /// # Returns
    /// `true` if the bit at the specified index is 1, `false` if it is 0.
    fn get_bit(compressed_bitstring: &str, bit_index: usize) -> Result<bool> {
        // Base64-decode the compressed bitstring
        let compressed_data = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(compressed_bitstring)
            .map_err(|e| Web5Error::Parameter(format!("failed to decode base64: {}", e)))?;

        // Decompress the data using GZIP
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut decompressed_data = Vec::new();
        decoder
            .read_to_end(&mut decompressed_data)
            .map_err(|e| Web5Error::Parameter(format!("failed to decompress data: {}", e)))?;

        // Find the byte index, and bit index within the byte
        let byte_index = bit_index / 8;
        let bit_index_within_byte = 7 - (bit_index % 8);
        let byte = decompressed_data.get(byte_index).ok_or_else(|| {
            Web5Error::Parameter("bit index out of range in decompressed data".into())
        })?;

        // Extract the targeted bit
        let bit_integer = (byte >> bit_index_within_byte) & 1;

        Ok(bit_integer == 1)
    }

    /// Helper function to extract a string property from the additional_properties
    fn get_additional_property<'a>(props: &'a Option<JsonObject>, key: &str) -> Result<&'a str> {
        props
            .as_ref()
            .and_then(|p| p.properties.get(key))
            .and_then(|value| {
                if let JsonValue::String(s) = value {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .ok_or_else(|| Web5Error::Parameter(format!("no valid {} found", key)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::verifiable_credential_1_1::{
        CredentialStatus, BASE_CONTEXT, BASE_TYPE,
    };

    const ISSUER_DID_URI: &str = "did:web:tbd.website";
    const SUBJECT_DID_URI: &str = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";

    fn issuer() -> Issuer {
        Issuer::from(ISSUER_DID_URI)
    }

    fn credential_subject() -> CredentialSubject {
        CredentialSubject::from(SUBJECT_DID_URI)
    }

    fn create_test_credential(index: &str, purpose: &str) -> VerifiableCredential {
        let credential_status = CredentialStatus {
            id: format!("https://example.com/status/{}", index),
            r#type: STATUS_LIST_2021_ENTRY.to_string(),
            status_purpose: purpose.to_string(),
            status_list_index: index.to_string(),
            status_list_credential: "https://example.com/status/1".to_string(),
        };

        VerifiableCredential::create(
            Issuer::from("did:example:issuer"),
            CredentialSubject::from("did:example:subject"),
            Some(VerifiableCredentialCreateOptions {
                credential_status: Some(credential_status),
                ..Default::default()
            }),
        )
        .unwrap()
    }

    fn create_test_credential_with_type(
        index: &str,
        status_type: &str,
        purpose: &str,
    ) -> VerifiableCredential {
        let mut credential = create_test_credential(index, purpose);
        if let Some(status) = &mut credential.credential_status {
            status.r#type = status_type.to_string();
        }
        credential
    }

    #[test]
    fn test_create_status_list_credential2() {
        let issuer = Issuer::from("did:example:123".to_string());
        let status_purpose = "revocation".to_string();
        let credentials_to_disable = None;

        let result = StatusListCredential::create(issuer, status_purpose, credentials_to_disable);

        assert!(result.is_ok());
        let status_list_credential = result.unwrap();

        assert_eq!(
            status_list_credential.base.r#type,
            vec![
                BASE_TYPE.to_string(),
                STATUS_LIST_CREDENTIAL_TYPE.to_string()
            ]
        );
        assert_eq!(
            status_list_credential.base.context,
            vec![
                BASE_CONTEXT.to_string(),
                STATUS_LIST_CREDENTIAL_CONTEXT.to_string()
            ]
        );

        let additional_properties = status_list_credential
            .base
            .credential_subject
            .additional_properties
            .unwrap();

        assert_eq!(
            additional_properties
                .properties
                .get("statusPurpose")
                .unwrap(),
            &JsonValue::String("revocation".to_string())
        );
        assert_eq!(
            additional_properties.properties.get("type").unwrap(),
            &JsonValue::String(STATUS_LIST_2021.to_string())
        );
        assert!(additional_properties
            .properties
            .get("encodedList")
            .is_some());
    }

    #[test]
    fn test_get_bit() {
        let bit_indices = vec![3, 1023];

        let bitstring = StatusListCredential::bitstring_generation(bit_indices).unwrap();

        assert_eq!(StatusListCredential::get_bit(&bitstring, 3).unwrap(), true);
        assert_eq!(
            StatusListCredential::get_bit(&bitstring, 1023).unwrap(),
            true
        );
        assert_eq!(StatusListCredential::get_bit(&bitstring, 0).unwrap(), false);
        assert_eq!(
            StatusListCredential::get_bit(&bitstring, 1024).unwrap(),
            false
        );

        let result = StatusListCredential::get_bit(&bitstring, 16 * 1024 * 8 + 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_disabled() -> Result<()> {
        // Create a StatusListCredential with some disabled credentials
        let issuer = Issuer::from("did:example:issuer");
        let status_purpose = "revocation".to_string();
        let credentials_to_disable = Some(vec![
            create_test_credential("3", &status_purpose),
            create_test_credential("1023", &status_purpose),
        ]);
        let status_list_credential =
            StatusListCredential::create(issuer, status_purpose.clone(), credentials_to_disable)?;

        // Test 1: Check a disabled credential (index 3)
        let disabled_credential = create_test_credential("3", &status_purpose);
        assert!(status_list_credential.is_disabled(&disabled_credential)?);

        // Test 2: Check another disabled credential (index 1023)
        let another_disabled_credential = create_test_credential("1023", &status_purpose);
        assert!(status_list_credential.is_disabled(&another_disabled_credential)?);

        // Test 3: Check an enabled credential (index 5)
        let enabled_credential = create_test_credential("5", &status_purpose);
        assert!(!status_list_credential.is_disabled(&enabled_credential)?);

        // Test 4: Check a credential with mismatched status type
        let mismatched_type_credential =
            create_test_credential_with_type("7", "InvalidType", &status_purpose);
        assert!(status_list_credential
            .is_disabled(&mismatched_type_credential)
            .is_err());

        // Test 5: Check a credential with mismatched status purpose
        let mismatched_purpose_credential = create_test_credential("9", "suspension");
        assert!(status_list_credential
            .is_disabled(&mismatched_purpose_credential)
            .is_err());

        // Test 6: Check a credential without a status
        let no_status_credential = VerifiableCredential::create(
            Issuer::from("did:example:issuer"),
            CredentialSubject::from("did:example:subject"),
            None,
        )?;
        assert!(status_list_credential
            .is_disabled(&no_status_credential)
            .is_err());

        Ok(())
    }

    #[test]
    fn test_full_flow() {
        let status_purpose = "revocation".to_string();
        let credentials_to_disable = None;
        let status_list_credential =
            StatusListCredential::create(issuer(), status_purpose, credentials_to_disable).unwrap();

        let encoded_list = StatusListCredential::get_additional_property(
            &status_list_credential
                .base
                .credential_subject
                .additional_properties,
            "encodedList",
        )
        .unwrap();

        // Test various bit positions
        assert_eq!(
            StatusListCredential::get_bit(encoded_list, 0).unwrap(),
            false
        );
        assert_eq!(
            StatusListCredential::get_bit(encoded_list, 100).unwrap(),
            false
        );
        assert_eq!(
            StatusListCredential::get_bit(encoded_list, 1000).unwrap(),
            false
        );

        let vc1_options = Some(VerifiableCredentialCreateOptions {
            credential_status: Some(CredentialStatus {
                id: "https://example.com/status/1".to_string(),
                r#type: STATUS_LIST_2021_ENTRY.to_string(),
                status_purpose: "revocation".to_string(),
                status_list_index: "3".to_string(),
                status_list_credential: "https://example.com/status/1".to_string(),
            }),
            ..Default::default()
        });

        let vc1 =
            VerifiableCredential::create(issuer(), credential_subject(), vc1_options).unwrap();

        let vc2_options = Some(VerifiableCredentialCreateOptions {
            credential_status: Some(CredentialStatus {
                id: "https://example.com/status/2".to_string(),
                r#type: STATUS_LIST_2021_ENTRY.to_string(),
                status_purpose: "revocation".to_string(),
                status_list_index: "1023".to_string(),
                status_list_credential: "https://example.com/status/1".to_string(),
            }),
            ..Default::default()
        });

        let vc2 =
            VerifiableCredential::create(issuer(), credential_subject(), vc2_options).unwrap();

        let credentials_to_disable = Some(vec![vc1, vc2]);

        let updated_status_list_credential = StatusListCredential::create(
            Issuer::from("did:example:123".to_string()),
            "revocation".to_string(),
            credentials_to_disable,
        )
        .unwrap();

        let updated_encoded_list = StatusListCredential::get_additional_property(
            &updated_status_list_credential
                .base
                .credential_subject
                .additional_properties,
            "encodedList",
        )
        .unwrap();

        // Test the bits corresponding to the disabled credentials
        assert_eq!(
            StatusListCredential::get_bit(updated_encoded_list, 3).unwrap(),
            true
        );
        assert_eq!(
            StatusListCredential::get_bit(updated_encoded_list, 1023).unwrap(),
            true
        );

        // Test other bits are still false
        assert_eq!(
            StatusListCredential::get_bit(updated_encoded_list, 0).unwrap(),
            false
        );
        assert_eq!(
            StatusListCredential::get_bit(updated_encoded_list, 100).unwrap(),
            false
        );
        assert_eq!(
            StatusListCredential::get_bit(updated_encoded_list, 1000).unwrap(),
            false
        );
    }
}
