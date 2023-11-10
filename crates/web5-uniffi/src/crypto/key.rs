use ssi_jwk::JWK;

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

impl Key for PrivateKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}
