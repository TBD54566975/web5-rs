use std::sync::Arc;

pub fn hello_world() {
    println!("Hello web5 :)")
}

pub trait Signer {
    fn sign(&self);
}

struct LocalSigner {}

impl Signer for LocalSigner {
    fn sign(&self) {
        println!("hello world")
    }
}

pub fn get_signer() -> Arc<dyn Signer> {
    Arc::new(LocalSigner {})
}

pub fn use_signer(signer: Arc<dyn Signer>) {
    signer.sign()
}

uniffi::include_scaffolding!("web5");
