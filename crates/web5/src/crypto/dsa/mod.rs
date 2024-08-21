use crate::errors::{Result, Web5Error};

pub mod ed25519;
pub mod secp256k1;

pub enum Dsa {
    Ed25519,
    #[cfg(test)]
    Secp256k1,
}

impl std::str::FromStr for Dsa {
    type Err = Web5Error;

    fn from_str(input: &str) -> std::result::Result<Self, Web5Error> {
        match input.to_ascii_lowercase().as_str() {
            "ed25519" => Ok(Dsa::Ed25519),
            #[cfg(test)]
            "secp256k1" => Ok(Dsa::Secp256k1),
            _ => Err(Web5Error::Parameter(format!("unsupported dsa {}", input))),
        }
    }
}

pub trait Signer: Send + Sync {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<()>;
}
