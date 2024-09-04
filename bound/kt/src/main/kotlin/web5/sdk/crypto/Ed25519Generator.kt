package web5.sdk.crypto

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.ed25519GeneratorGenerate
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Generates private key material for Ed25519.
 */
class Ed25519Generator {
    companion object {
        /**
         * Generate the private key material; return Jwk includes private key material.
         *
         * @return Jwk the JWK with private key material included.
         */
        fun generate(): Jwk {
            try {
                val rustCoreJwkData = ed25519GeneratorGenerate()
                return Jwk.fromRustCoreJwkData(rustCoreJwkData)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }
}