use std::collections::HashMap;

use crate::crypto::dsa::{secp256k1, x25519};
use crate::{crypto::dsa::ed25519, dids::data_model::verification_method::VerificationMethod};
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
    pub fn record_name(idx: u32) -> String {
        format!("_k{}._did", idx)
    }

    pub fn is_vm_record_with_index(record: &ResourceRecord, idx: &u32) -> bool {
        let labels = record.name.get_labels();

        match labels.first() {
            None => return false,
            Some(subdomain) => {
                if subdomain.to_string() != format!("_k{}", idx) {
                    return false;
                }
            }
        };

        return match labels.get(1) {
            None => false,
            Some(subdomain) => subdomain.to_string() == "_did",
        };
    }

    pub fn to_resource_record(
        &self,
        did_uri: &str,
        idx: u32,
    ) -> Result<ResourceRecord, DocumentPacketError> {
        let curve = match self.public_key_jwk.crv.as_str() {
            "Ed25519" => "Ed25519",
            "secp256k1" => "secp256k1",
            "X25519" => "X25519",
            // TODO: support remaining key types in key type index registry https://did-dht.com/registry/index.html#key-type-index
            "secp256r1" => {
                return Err(DocumentPacketError::PublicKeyJwk(
                    "Could not extract public key because curve is not yet supported".to_string(),
                ))
            }
            _ => return Err(DocumentPacketError::PublicKeyJwk(
                "Curve not allowed for did:dht because it does not appear in the key type registry"
                    .to_string(),
            )),
        };

        let key_type_idx = match curve {
            "Ed25519" => '0',
            "secp256k1" => '1',
            "X25519" => '3',
            _ => unreachable!(),
        };

        let public_key_bytes = match curve {
            "Ed25519" => ed25519::public_jwk_extract_bytes(&self.public_key_jwk)?,
            "secp256k1" => secp256k1::public_jwk_extract_bytes(&self.public_key_jwk)?,
            "X25519" => x25519::public_jwk_extract_bytes(&self.public_key_jwk)?,
            _ => unreachable!(),
        };
        let k = general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes);

        let default_alg = match curve {
            "Ed25519" => "Ed25519",
            "secp256k1" => "ES256K",
            "X25519" => "ECDH-ES+A256KW",
            _ => unreachable!(),
        };

        let mut parts = vec![format!("t={}", key_type_idx), format!("k={}", k)];
        if did_uri != self.controller {
            parts.push(format!("c={}", self.controller));
        }
        if let Some(alg) = &self.public_key_jwk.alg {
            if default_alg != alg {
                parts.push(format!("a={}", alg));
            }
        }
        let parts = parts.join(";");

        let name = Name::new_unchecked(&VerificationMethod::record_name(idx)).into_owned();
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
        record: &ResourceRecord,
        identity_key: bool,
    ) -> Result<Self, DocumentPacketError> {
        let vm_rdata: VerificationMethodRdata = record_rdata_to_hash_map(record)?.try_into()?;

        let curve = match vm_rdata.t.as_str() {
            "0" => "Ed25519",
            "1" => "secp256k1",
            "3" => "X25519",
            // TODO: support remaining key types in key type index registry https://did-dht.com/registry/index.html#key-type-index
            "2" => return Err(DocumentPacketError::PublicKeyJwk(
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
            "Ed25519" => ed25519::public_jwk_from_bytes(&public_key_bytes)?,
            "secp256k1" => secp256k1::public_jwk_from_bytes(&public_key_bytes)?,
            "X25519" => x25519::public_jwk_from_bytes(&public_key_bytes)?,
            _ => unreachable!(),
        };
        public_key_jwk.alg = if let Some(alg) = vm_rdata.a {
            Some(alg)
        } else {
            match public_key_jwk.crv.as_str() {
                "secp256k1" => Some("ES256K".to_string()),
                "Ed25519" => Some("Ed25519".to_string()),
                "X25519" => Some("ECDH-ES+A256KW".to_string()),
                _ => public_key_jwk.alg,
            }
        };

        let id_fragment = if identity_key {
            "0".to_string()
        } else {
            public_key_jwk.compute_thumbprint()?
        };

        let controller = vm_rdata.c.unwrap_or(did_uri.to_string());

        Ok(VerificationMethod {
            id: format!("{}#{}", did_uri, id_fragment),
            r#type: "JsonWebKey".to_string(),
            controller,
            public_key_jwk,
        })
    }
}

#[cfg(test)]
mod tests {
    use ed25519::Ed25519Generator;
    use simple_dns::rdata::A;
    use x25519_dalek::{EphemeralSecret, PublicKey};

    use crate::crypto::jwk::Jwk;

    use super::*;

    #[test]
    fn test_to_and_from_resource_record_ed25519() {
        let did_uri = "did:dht:123";
        let public_key_jwk = ed25519::to_public_jwk(&Ed25519Generator::generate());
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
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, &record, false)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_and_from_resource_record_identity_key() {
        let did_uri = "did:dht:123";
        let public_key_jwk = ed25519::to_public_jwk(&Ed25519Generator::generate());
        let id = format!("{}#0", did_uri);
        let vm = VerificationMethod {
            id,
            r#type: "JsonWebKey".to_string(),
            controller: did_uri.to_string(),
            public_key_jwk,
        };

        let record = vm
            .to_resource_record(&did_uri, 0)
            .expect("Expected to convert verification method to DNS record");
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, &record, true)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_and_from_resource_record_secp256k1() {
        let did_uri = "did:dht:123";
        let public_key_jwk = secp256k1::to_public_jwk(&secp256k1::Secp256k1Generator::generate());
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
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, &record, false)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_and_from_resource_record_x25519() {
        let private_key = EphemeralSecret::random();
        let public_key = PublicKey::from(&private_key);
        let x = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());

        let public_key_jwk = Jwk {
            alg: Some("ECDH-ES+A256KW".to_string()),
            kty: "OKP".to_string(),
            crv: "X25519".to_string(),
            d: None,
            x,
            y: None,
        };

        let did_uri = "did:dht:123";
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
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, &record, false)
            .expect("Expected to convert DNS record back to verification method");
        assert_eq!(vm, reconstituted_vm);
    }

    #[test]
    fn test_to_resource_record_unsupported_key_type() {
        let did_uri = "did:dht:123";
        let mut public_key_jwk = ed25519::to_public_jwk(&Ed25519Generator::generate());
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

        VerificationMethod::from_resource_record("did:dht:123", &record, false)
            .expect_err("Expected to fail because key type index is not supported");
    }

    #[test]
    fn test_to_and_from_resource_record_non_default_alg() {
        let did_uri = "did:dht:123";
        let mut public_key_jwk = ed25519::to_public_jwk(&Ed25519Generator::generate());
        public_key_jwk.alg = Some("nonstandard".to_string());
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
        let reconstituted_vm = VerificationMethod::from_resource_record(did_uri, &record, false)
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

        let error = VerificationMethod::from_resource_record("did:dht:123", &record, false)
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

        let error = VerificationMethod::from_resource_record("did:dht:123", &record, false)
            .expect_err("Expected to fail because RData TXT is malformed");
        match error {
            DocumentPacketError::RDataError(_) => {}
            _ => panic!("Expected error to be DocumentPacketError::RDataError"),
        }
    }
}
