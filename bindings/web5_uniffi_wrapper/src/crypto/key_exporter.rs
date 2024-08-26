use crate::errors::Result;
use std::sync::Arc;
use web5::crypto::{jwk::Jwk, key_managers::KeyExporter as InnerKeyExporter};

pub trait KeyExporter: Send + Sync {
    fn export_private_jwks(&self) -> Result<Vec<Jwk>>;
}

pub struct ToInnerKeyExporter(pub Arc<dyn KeyExporter>);

impl InnerKeyExporter for ToInnerKeyExporter {
    fn export_private_jwks(&self) -> web5::errors::Result<Vec<Jwk>> {
        let private_jwks = self.0.export_private_jwks()?;
        Ok(private_jwks)
    }
}
