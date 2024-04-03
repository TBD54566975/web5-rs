use base64::{engine::general_purpose, Engine};
use crypto::key_manager::KeyManager;
use did_core::{bearer_did::BearerDid, did::Did, document::VerificationMethodSelector};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
}

impl Header {
    pub fn encode(&self) -> Result<String, String> {
        let header_json = serde_json::to_string(&self).unwrap();
        let base64_url_encoded_header =
            general_purpose::URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        Ok(base64_url_encoded_header)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Decoded {
    pub header: Header,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub parts: Vec<String>,
}

#[derive(Default)]
pub struct DecodeOptions {
    pub detached_payload: Option<Vec<u8>>,
}

pub fn decode(jws: &str, options: DecodeOptions) -> Result<Decoded, String> {
    let mut parts: Vec<String> = jws.split('.').map(|s| s.to_string()).collect();
    if parts.len() != 3 {
        return Err(format!(
            "malformed JWS. Expected 3 parts, got {}",
            parts.len()
        ));
    }

    let header = decode_header(&parts[0])?;

    let payload = if let Some(p) = options.detached_payload {
        parts[1] = general_purpose::URL_SAFE_NO_PAD.encode(&p);
        p
    } else {
        general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|e| format!("malformed JWS. Failed to decode payload: {}", e))?
    };

    let signature = general_purpose::URL_SAFE_NO_PAD
        .decode(&parts[2])
        .map_err(|e| format!("malformed JWS. Failed to decode signature: {}", e))?;

    Ok(Decoded {
        header,
        payload,
        signature,
        parts: parts.into_iter().map(String::from).collect(),
    })
}

pub fn decode_header(encoded_header: &str) -> Result<Header, String> {
    let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(encoded_header)
        .map_err(|e| format!("Failed to decode header: {}", e))?;
    serde_json::from_slice::<Header>(&decoded_bytes)
        .map_err(|e| format!("Failed to parse header: {}", e))
}

pub fn verify(jws: &str, options: DecodeOptions) -> Result<Decoded, String> {
    let decoded =
        decode(jws, options).map_err(|e| format!("Unable to acquire Mutex lock: {}", e))?;

    Ok(decoded)
}

impl Decoded {
    pub fn verify(&self) -> Result<(), String> {
        if self.header.alg.is_none() {
            return Err("alg is required".into());
        }

        let did = match &self.header.kid {
            Some(kid) => Did::parse(kid),
            None => return Err("kid is required".into())
        };

        // TODO resolve did doc
        // TODO extend key manager to perform verification

        // let resolution_result = dids::resolve(&did.uri)?;
        // if resolution_result.is_err() {
        //     return Err(format!("failed to resolve DID: {}", resolution_result.unwrap_err()).into());
        // }

        // let vm_selector = didcore::ID(did.url);
        // let verification_method = resolution_result.document.select_verification_method(vm_selector)?;
        // if verification_method.is_err() {
        //     return Err(format!("kid does not match any verification method {}", verification_method.unwrap_err()).into());
        // }

        // let to_verify = format!("{}.{}", self.parts[0], self.parts[1]);

        // let verified = dsa::verify(to_verify.as_bytes(), &self.signature, &verification_method.public_key_jwk)?;
        // if verified.is_err() {
        //     return Err(format!("failed to verify signature: {}", verified.unwrap_err()).into());
        // }

        // if !verified.unwrap() {
        //     return Err("invalid signature".into());
        // }

        Ok(())
    }
}

#[derive(Default)]
pub struct SignOptions {
    pub selector: Option<VerificationMethodSelector>,
    pub detached: bool,
    pub typ: Option<String>,
}

pub fn sign<T: KeyManager>(
    payload: &[u8],
    did: &BearerDid<T>,
    options: SignOptions,
) -> Result<String, String> {
    let vm = did.document.select_verification_method(options.selector)?;
    let key_alias = did.key_manager.alias(&vm.public_key_jwk).unwrap();

    let header = Header {
        alg: Some(String::from("todo")),
        kid: Some(vm.id),
        typ: options.typ,
    };
    let encoded_header = header.encode()?;
    let encoded_payload = general_purpose::URL_SAFE_NO_PAD.encode(payload);

    let signature_bytes = did
        .key_manager
        .sign(
            &key_alias,
            format!("{}.{}", encoded_header, encoded_payload).as_bytes(),
        )
        .unwrap();
    let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature_bytes);

    let compact_jws = if options.detached {
        format!("{}.{}", encoded_header, encoded_signature)
    } else {
        format!(
            "{}.{}.{}",
            encoded_header, encoded_payload, encoded_signature
        )
    };

    Ok(compact_jws)
}

#[cfg(test)]
mod tests {
    use crypto::{key::KeyType, key_manager::local_key_manager::LocalKeyManager};
    use did_core::{
        did::Did,
        document::{Document, VerificationMethod},
    };

    use super::*;

    #[test]
    fn test_decode() {
        let valid_jws = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJa2t0WTA5R1NHRmhXalZLYlRSWmJISjBVVFJJUkZFdFJGaFZNMHRuYTI5NWJVTjBiazlhY1ZWSmEyOGlmUSMwIn0.eyJoZWxsbyI6IndvcmxkIiwiaXNzIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpZUNJNklra3RZMDlHU0dGaFdqVktiVFJaYkhKMFVUUklSRkV0UkZoVk0wdG5hMjk1YlVOMGJrOWFjVlZKYTI4aWZRIn0.K3c3_Vq9wSOWIjkkVJkCACl0KSpNUey1hRT-9d3Czl6dCg7rzfGPBqrz97CAEBKTIabtMNZBUstkcY6jXIKhBw";

        let decoded = decode(valid_jws, DecodeOptions::default()).unwrap();

        assert_eq!(
            decoded.header,
            Header {
                alg: Some("EdDSA".to_string()),
                kid: Some("did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6IkktY09GSGFhWjVKbTRZbHJ0UTRIRFEtRFhVM0tna295bUN0bk9acVVJa28ifQ#0".to_string()),
                typ: None,
            }
        );

        let expected_payload = r#"{"hello":"world","iss":"did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6IkktY09GSGFhWjVKbTRZbHJ0UTRIRFEtRFhVM0tna295bUN0bk9acVVJa28ifQ"}"#;
        assert_eq!(
            std::str::from_utf8(&decoded.payload).unwrap(),
            expected_payload
        );

        let expected_signature = [
            43, 119, 55, 253, 90, 189, 193, 35, 150, 34, 57, 36, 84, 153, 2, 0, 41, 116, 41, 42,
            77, 81, 236, 181, 133, 20, 254, 245, 221, 194, 206, 94, 157, 10, 14, 235, 205, 241,
            143, 6, 170, 243, 247, 176, 128, 16, 18, 147, 33, 166, 237, 48, 214, 65, 82, 203, 100,
            113, 142, 163, 92, 130, 161, 7,
        ];
        assert_eq!(decoded.signature, expected_signature);

        assert_eq!(
            decoded.parts,
            vec![
                "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aWVDSTZJa2t0WTA5R1NHRmhXalZLYlRSWmJISjBVVFJJUkZFdFJGaFZNMHRuYTI5NWJVTjBiazlhY1ZWSmEyOGlmUSMwIn0",
                "eyJoZWxsbyI6IndvcmxkIiwiaXNzIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpZUNJNklra3RZMDlHU0dGaFdqVktiVFJaYkhKMFVUUklSRkV0UkZoVk0wdG5hMjk1YlVOMGJrOWFjVlZKYTI4aWZRIn0",
                "K3c3_Vq9wSOWIjkkVJkCACl0KSpNUey1hRT-9d3Czl6dCg7rzfGPBqrz97CAEBKTIabtMNZBUstkcY6jXIKhBw"
            ]
        );
    }

    #[test]
    fn test_sign() {
        let key_manager = LocalKeyManager::new_in_memory();
        let did = Did::parse("did:example:123").unwrap();
        let key_alias = key_manager
            .generate_private_key(KeyType::Secp256k1)
            .unwrap();

        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("KeyManagerError occurred")
            .expect("PublicKey not found");

        let method = VerificationMethod {
            id: format!("{}#{}", did, key_alias),
            controller: did.uri.to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key.clone(),
        };

        let document = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![method.clone()],
            ..Default::default()
        };

        let bearer_did = BearerDid {
            did: did.clone(),
            key_manager,
            document,
        };

        let payload = b"Hello, world!";
        let options = SignOptions {
            selector: Some(VerificationMethodSelector::ID(method.id)),
            detached: false,
            typ: Some("JWT".to_string()),
        };

        let jws = sign(payload, &bearer_did, options).unwrap();

        let decoded = decode(&jws, DecodeOptions::default()).unwrap();
        assert_eq!(decoded.header.typ, Some("JWT".to_string()));
        assert_eq!(
            std::str::from_utf8(&decoded.payload).unwrap(),
            "Hello, world!"
        );
    }
}
