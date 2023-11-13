use ssi_jwk::JWK;
use std::sync::Arc;

#[derive(uniffi::Enum)]
pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}

pub trait Key {
    fn alias(&self) -> KeyAlias;
}

pub type KeyAlias = String;

#[derive(uniffi::Object, Clone)]
pub struct PrivateKey(pub JWK);

#[uniffi::export]
impl Key for PrivateKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}

#[uniffi::export]
impl PrivateKey {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    pub fn sign(&self, payload: Vec<u8>) -> Vec<u8> {
        let algorithm = self
            .0
            .get_algorithm()
            .expect("Expected algorithm to be present");
        let signed_bytes = ssi_jws::sign_bytes(algorithm, &payload, &self.0)
            .expect("Signature not computed properly");

        signed_bytes
    }
}
