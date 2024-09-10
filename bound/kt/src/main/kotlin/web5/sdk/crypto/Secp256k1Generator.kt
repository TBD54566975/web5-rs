package web5.sdk.crypto

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.secp256k1GeneratorGenerate
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Generates private key material for secp256k1.
 */
class Secp256k1Generator {
    companion object {
        /**
         * Generate the private key material; return Jwk includes private key material.
         *
         * @return Jwk the JWK with private key material included.
         */
        fun generate(): Jwk {
            try {
                val rustCoreJwkData = secp256k1GeneratorGenerate()
                return Jwk.fromRustCoreJwkData(rustCoreJwkData)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }
}