// 🚧 [KW] no errors whatsoever defined here, but needed

pub mod jwk {
    pub struct Jwk {
        // 🚧
    }
}

pub mod dsa {
    use crate::jwk::Jwk;

    pub enum Dsa {
        Ed25519,
        // 🚧 Xd25519, Secp256k1, Secp256r1
    }

    /// private key material is encapsulated in impl
    pub trait DsaSigner {
        fn sign(self, payload: &[u8]) -> Vec<u8>;
    }

    /// public key material is encapsulated in impl
    pub trait DsaVerifier {
        fn verify(self, message: &[u8], signature: &[u8]);
    }

    /// private key material is encapsulated in impl
    pub trait JwsSigner {
        fn sign(self, payload: &[u8]) -> Vec<u8>;
    }

    /// public key material is encapsulated in impl
    pub trait JwsVerifier {
        fn verify(self, message: &[u8], signature: &[u8]);
    }

    pub struct Ed25519 {
        key: Jwk,
    }

    impl Ed25519 {
        pub fn new() -> Jwk {
            // 🚧 [KW] use this, but map onto our Jwk struct https://github.com/hidekatsu-izuno/josekit-rs/blob/b13b6fecd4e7b2dfc5af0f2e7cbd726791e66a8a/src/jwk/alg/ed.rs#L77
            unimplemented!()
        }

        pub fn from_key(key: Jwk) -> Self {
            Self { key }
        }
    }

    impl DsaSigner for Ed25519 {
        fn sign(self, payload: &[u8]) -> Vec<u8> {
            // 🚧 [KW] using https://github.com/hidekatsu-izuno/josekit-rs/blob/b13b6fecd4e7b2dfc5af0f2e7cbd726791e66a8a/src/jws/alg/eddsa.rs#L298
            unimplemented!()
        }
    }

    impl DsaVerifier for Ed25519 {
        fn verify(self, message: &[u8], signature: &[u8]) {
            // 🚧 [KW] using https://github.com/hidekatsu-izuno/josekit-rs/blob/b13b6fecd4e7b2dfc5af0f2e7cbd726791e66a8a/src/jws/alg/eddsa.rs#L350
            unimplemented!()
        }
    }

    impl JwsSigner for Ed25519 {
        fn sign(self, payload: &[u8]) -> Vec<u8> {
            // 🚧 [KW] using https://github.com/hidekatsu-izuno/josekit-rs/blob/b13b6fecd4e7b2dfc5af0f2e7cbd726791e66a8a/src/jws/jws_context.rs#L56
            //          can use default JwsHeader, need to override the `kid` though
            unimplemented!()
        }
    }

    impl JwsVerifier for Ed25519 {
        fn verify(self, message: &[u8], signature: &[u8]) {
            // 🚧 [KW] using https://github.com/hidekatsu-izuno/josekit-rs/blob/b13b6fecd4e7b2dfc5af0f2e7cbd726791e66a8a/src/jws/jws_context.rs#L378
            unimplemented!()
        }
    }
}

pub mod dids {
    pub struct Did {
        // 🚧 [KW] we currently call this Identifier, so it's just a rename
    }

    impl Did {
        pub fn parse(uri: &str) -> Self {
            unimplemented!()
        }
    }

    pub struct Document {
        // 🚧
    }

    pub struct VerificationMethod {
        // 🚧
    }

    pub struct Service {
        // 🚧
    }

    pub struct Resolution {
        // 🚧
    }

    pub struct DocumentMetadata {
        // 🚧
    }

    pub struct ResolutionMetadata {
        // 🚧
    }

    pub mod methods {
        use crate::jwk::Jwk;

        use super::{Did, Document, Resolution};

        pub struct DidJwk {
            pub did: Did,
            pub document: Document,
        }

        impl DidJwk {
            pub fn create(jwk: Jwk) -> Self {
                unimplemented!()
            }

            pub fn resolve(uri: &str) -> Resolution {
                unimplemented!()
            }
        }

        pub struct DidWeb {}

        impl DidWeb {
            pub fn resolve(uri: &str) -> Resolution {
                unimplemented!()
            }
        }

        pub struct DidDht {
            pub did: Did,
            pub document: Document,
        }

        impl DidDht {
            pub fn resolve(uri: &str) -> Resolution {
                // 🚧 [KW] can use built in Ed25519's verify instance method
                unimplemented!()
            }

            // 🚧 create
            // 🚧 update & deactivate
        }
    }
}

pub mod credentials {
    use crate::dsa::{JwsSigner, JwsVerifier};

    pub struct VerifiableCredential {
        // 🚧
    }

    // 🚧 [KW] existing things around NamedIssuer and whatnot

    impl VerifiableCredential {
        pub fn sign(self, jws_signer: &dyn JwsSigner) -> String {
            unimplemented!()
        }

        pub fn verify(vcjwt: &str) {
            // 🚧 [KW] use set of built-in JwsVerifier implementations, currently only Ed25519
        }

        pub fn verify_with_verifier(vcjwt: &str, jws_verifier: &dyn JwsVerifier) {
            unimplemented!()
        }
    }
}

pub mod bearer_did {
    use crate::credentials::VerifiableCredential;

    pub struct BearerDid {
        // 🚧
    }

    pub struct KeySelector {
        // 🚧
    }

    pub struct LocalKeyManager {}

    pub enum Method {
        Jwk,
        Dht,
    }

    pub struct CreateOptions {
        method: Method,
        key_manager: Option<LocalKeyManager>,
    }

    impl BearerDid {
        pub fn create(options: CreateOptions) -> Self {
            unimplemented!()
        }

        pub fn sign_vcjwt(vc: VerifiableCredential, key_selector: Option<KeySelector>) -> String {
            // 🚧 [KW] default to first vm
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod tests {
        fn test_create_did_jwk() {
            // 🚧
        }

        fn test_create_did_dht() {
            // 🚧
        }
    }
}
