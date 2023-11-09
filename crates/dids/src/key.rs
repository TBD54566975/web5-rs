use ssi_jwk::JWK;

#[derive(uniffi::Enum)]
pub enum KeyAlgorithm {
    Secp256k1,
    Secp256r1,
    Ed25519,
}
pub struct PublicKey(pub JWK);
#[derive(uniffi::Object)]
pub struct PrivateKey(pub JWK);

impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        PublicKey(self.0.to_public())
    }
}
