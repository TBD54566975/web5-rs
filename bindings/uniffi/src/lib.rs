use jwk::{Jwk, JwkError};

pub fn hello_world() {
    println!("Hello web5 :)")
}

pub fn jwk_compute_thumbprint(jwk: Jwk) -> Result<String, JwkError> {
    jwk.compute_thumbprint()
}

uniffi::include_scaffolding!("web5");
