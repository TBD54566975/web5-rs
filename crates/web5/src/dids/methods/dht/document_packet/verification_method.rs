use std::{collections::HashMap, str::FromStr};

use crate::dids::document::VerificationMethod;

use crate::crypto::{ed25519::Ed25519, secp256k1::Secp256k1, Curve};
use base64::{engine::general_purpose, Engine as _};
use simple_dns::{
    rdata::{RData, TXT},
    Name, ResourceRecord,
};

use super::{
    rdata_encoder::{get_rdata_txt_value, record_rdata_to_hash_map},
    DocumentPacketError, DEFAULT_TTL,
};

#[derive(Debug, PartialEq)]
struct VerificationMethodRdata {
    pub t: String,
    pub k: String,
    pub c: Option<String>,
    pub a: Option<String>,
}

impl TryFrom<HashMap<String, String>> for VerificationMethodRdata {
    fn try_from(rdata_map: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(VerificationMethodRdata {
            t: get_rdata_txt_value(&rdata_map, "t")?,
            k: get_rdata_txt_value(&rdata_map, "k")?,
            c: get_rdata_txt_value(&rdata_map, "c").ok(),
            a: get_rdata_txt_value(&rdata_map, "a").ok(),
        })
    }

    type Error = DocumentPacketError;
}

impl VerificationMethod {
    pub fn to_resource_record(
        &self,
        did_uri: &str,
        idx: u32,
    ) -> Result<ResourceRecord, DocumentPacketError> {
        let curve = match self.public_key_jwk.crv.as_str() {
            // TODO: support remaining key types in key type index registry https://did-dht.com/registry/index.html#key-type-index
            "secp256r1" => return Err(DocumentPacketError::PublicKeyJwk(
                "Could not extract public key because curve is not yet supported".to_string()
            )),
            "X25519" => return Err(
                DocumentPacketError::PublicKeyJwk("Could not extract public key because curve is not yet supported".to_string()
            )),
            _ => Curve::from_str(&self.public_key_jwk.crv)
                    .map_err(|_| DocumentPacketError::PublicKeyJwk(
                        "Curve not allowed for did:dht because it does not appear in the key type registry".to_string()
                    ))?
        };

        let key_type_idx = match curve {
            Curve::Ed25519 => '0',
            Curve::Secp256k1 => '1',
        };

        let public_key_bytes = match curve {
            Curve::Ed25519 => Ed25519::extract_public_key(&self.public_key_jwk)?,
            Curve::Secp256k1 => Secp256k1::extract_public_key(&self.public_key_jwk)?,
        };
        let k = general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes);

        let default_alg = match curve {
            Curve::Secp256k1 => "ES256K",
            Curve::Ed25519 => "Ed25519",
        };

        let mut parts = vec![format!("t={}", key_type_idx), format!("k={}", k)];
        if did_uri != self.controller {
            parts.push(format!("c={}", self.controller));
        }
        if default_alg != self.public_key_jwk.alg {
            parts.push(format!("a={}", self.public_key_jwk.alg));
        }
        let parts = parts.join(";");

        let name = Name::new_unchecked(&format!("_k{}._did", idx)).into_owned();
        let txt_record = TXT::new().with_string(&parts)?.into_owned();

        Ok(ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        ))
    }

    pub fn from_resource_record(
        did_uri: &str,
        record: ResourceRecord,
    ) -> Result<Self, DocumentPacketError> {
        let vm_rdata: VerificationMethodRdata = record_rdata_to_hash_map(record)?.try_into()?;

        let curve = match vm_rdata.t.as_str() {
            "0" => Curve::Ed25519,
            "1" => Curve::Secp256k1,
            // TODO: support remaining key types in key type index registry https://did-dht.com/registry/index.html#key-type-index
            "2" => return Err(DocumentPacketError::PublicKeyJwk(
                "Could not reconstitute public jwk from DNS record because curve is not yet supported".to_string()
            )),
            "3" => return Err(DocumentPacketError::PublicKeyJwk(
                "Could not reconstitute public jwk from DNS record because curve is not yet supported".to_string()
            )),
            _ => return Err(DocumentPacketError::PublicKeyJwk(
                "Could not reconstitute public jwk from DNS record because key type does not appear in the did:dht key type registry".to_string()
            )),
        };

        let public_key_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(vm_rdata.k)
            .map_err(|_| {
                DocumentPacketError::PublicKeyJwk(
                    "Could not base64url decode k from DNS Record txt".to_string(),
                )
            })?;
        let mut public_key_jwk = match curve {
            Curve::Ed25519 => Ed25519::from_public_key(&public_key_bytes)?,
            Curve::Secp256k1 => Secp256k1::from_public_key(&public_key_bytes)?,
        };
        if let Some(alg) = vm_rdata.a {
            public_key_jwk.alg = alg;
        }

        let controller = vm_rdata.c.unwrap_or(did_uri.to_string());

        Ok(VerificationMethod {
            id: format!("{}#{}", did_uri, public_key_jwk.compute_thumbprint()?),
            r#type: "JsonWebKey".to_string(),
            controller,
            public_key_jwk,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::CurveOperations;
    use simple_dns::rdata::A;

    use super::*;

    #[test]
    fn test_to_and_from_resource_record_ed25519() {
        let did_uri = "did:dht:123";
        let public_key_jwk = Ed25519::generate().unwrap().to_public_jwk();
        let id = format!(
            "{}#{}",
            did_uri,
            public_key_jwk.compute_thumbprint().unwrap()
        );
        let vm = VerificationMethod {
            id,
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk,
        };

        let record = vm
            .to_resource_record(&did_uri, 0)
            .expect("Expected to convert verification method to DNS record");
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, record)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_and_from_resource_record_secp256k1() {
        let did_uri = "did:dht:123";
        let public_key_jwk = Secp256k1::generate().unwrap().to_public_jwk();
        let id = format!(
            "{}#{}",
            did_uri,
            public_key_jwk.compute_thumbprint().unwrap()
        );
        let vm = VerificationMethod {
            id,
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk,
        };

        let record = vm
            .to_resource_record(&did_uri, 0)
            .expect("Expected to convert verification method to DNS record");
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, record)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_resource_record_unsupported_key_type() {
        let did_uri = "did:dht:123";
        let mut public_key_jwk = Ed25519::generate().unwrap().to_public_jwk();
        public_key_jwk.crv = "nonsense".to_string();
        let id = format!(
            "{}#{}",
            did_uri,
            public_key_jwk.compute_thumbprint().unwrap()
        );
        let vm = VerificationMethod {
            id,
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk,
        };
        vm.to_resource_record(&did_uri, 0)
            .expect_err("Expected to fail because curve is not supported");
    }

    #[test]
    fn test_from_resource_record_unsupported_key_type() {
        let parts = format!("t=19;k=foo;a=bar;c=baz"); // "t" is not a supported key type index

        let name = Name::new_unchecked("_k0._did");
        let txt_record = TXT::new().with_string(&parts).unwrap().into_owned();

        let record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt_record),
        );

        VerificationMethod::from_resource_record("did:dht:123", record)
            .expect_err("Expected to fail because key type index is not supported");
    }

    #[test]
    fn test_to_and_from_resource_record_non_default_alg() {
        let did_uri = "did:dht:123";
        let mut public_key_jwk = Ed25519::generate().unwrap().to_public_jwk();
        public_key_jwk.alg = "nonstandard".to_string();
        let id = format!(
            "{}#{}",
            did_uri,
            public_key_jwk.compute_thumbprint().unwrap()
        );
        let vm = VerificationMethod {
            id,
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk,
        };

        let record = vm
            .to_resource_record(&did_uri, 0)
            .expect("Expected to convert verification method to DNS record");
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, record)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_from_resource_record_rdata_not_txt() {
        let name = Name::new_unchecked("_k0._did");
        let record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::A(A { address: 0 }), // not RData::TXT
        );

        let error = VerificationMethod::from_resource_record("did:dht:123", record)
            .expect_err("Expected to fail because RData is not TXT");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!("Expected error to be DocumentPacketError::RDataError"),
        }
    }

    #[test]
    fn test_from_resource_record_rdata_txt_malformed() {
        let name = Name::new_unchecked("_k0._did");
        let txt = TXT::new().with_string("a=b=c;;;").unwrap();
        let record = ResourceRecord::new(
            name,
            simple_dns::CLASS::IN,
            DEFAULT_TTL,
            RData::TXT(txt), // Not ';' separated entries
        );

        let error = VerificationMethod::from_resource_record("did:dht:123", record)
            .expect_err("Expected to fail because RData TXT is malformed");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!("Expected error to be DocumentPacketError::RDataError"),
        }
    }
}
