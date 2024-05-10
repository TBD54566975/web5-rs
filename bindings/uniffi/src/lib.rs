use jwk::{Jwk, JwkError};

pub fn hello_world() {
    println!("Hello web5 :)")
}

uniffi::include_scaffolding!("web5");
