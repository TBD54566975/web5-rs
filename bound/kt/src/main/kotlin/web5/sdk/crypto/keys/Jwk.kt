package web5.sdk.crypto.keys

import web5.sdk.Web5Exception
import web5.sdk.rust.Jwk as RustCoreJwk
import web5.sdk.rust.JwkData as RustCoreJwkData
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents a JSON Web Key (JWK) as per [RFC7517](https://tools.ietf.org/html/rfc7517).
 * This is a partial representation, containing key parameters such as `alg`, `kty`, `crv`, and key material like `x`, `y`, and `d`.
 *
 * @property alg The algorithm intended for use with the key (optional).
 * @property kty The key type (e.g., `EC`, `OKP`).
 * @property crv The curve used with the key (e.g., `Ed25519`, `X25519`).
 * @property x The `x` coordinate for elliptic curve keys, or the public key for OKP keys.
 * @property y The `y` coordinate for elliptic curve keys (optional).
 * @property d The private key material, if present (optional).
 */
data class Jwk (
    val alg: String? = null,
    val kty: String,
    val crv: String,
    val x: String,
    val y: String? = null,
    val d: String? = null
) {
    // Internal representation of the JWK for use with Rust core logic.
    internal val rustCoreJwkData: RustCoreJwkData = RustCoreJwkData(
        alg,
        kty,
        crv,
        d,
        x,
        y
    )

    companion object {
        /**
         * Converts a Rust core `JwkData` object into a Kotlin `Jwk` object.
         *
         * This is an internal method for transforming the Rust core JWK structure into the Kotlin equivalent.
         *
         * @param rustCoreJwkData The JWK data from Rust core.
         * @return A Kotlin `Jwk` object.
         */
        internal fun fromRustCoreJwkData(rustCoreJwkData: RustCoreJwkData): Jwk {
            return Jwk(
                rustCoreJwkData.alg,
                rustCoreJwkData.kty,
                rustCoreJwkData.crv,
                rustCoreJwkData.x,
                rustCoreJwkData.y,
                rustCoreJwkData.d,
            )
        }
    }

    /**
     * Computes the thumbprint of the JWK.
     *
     * A thumbprint is a cryptographic hash that uniquely identifies the JWK based on its key type and key material.
     * This method utilizes Rust core logic to compute the thumbprint.
     *
     * @return A base64url-encoded thumbprint string.
     * @throws Web5Exception If an error occurs during thumbprint computation.
     */
    fun computeThumbprint(): String {
        try {
            val rustCoreJwk = RustCoreJwk(rustCoreJwkData)
            return rustCoreJwk.computeThumbprint()
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
