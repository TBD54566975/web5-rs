package web5.sdk.crypto

import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.ed25519GeneratorGenerate

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
            val rustCoreJwkData = ed25519GeneratorGenerate()
            return Jwk.fromRustCoreJwkData(rustCoreJwkData)
        }
    }
}