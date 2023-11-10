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

#[derive(Clone, uniffi::Object)]
pub struct PublicKey(pub JWK);

impl Key for PublicKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}

#[derive(Clone, uniffi::Object)]
pub struct PrivateKey(pub JWK);

impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }
}

impl Key for PrivateKey {
    fn alias(&self) -> KeyAlias {
        self.0.thumbprint().unwrap()
    }
}
