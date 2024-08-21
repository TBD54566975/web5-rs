package web5.sdk.crypto.keys

import web5.sdk.rust.Jwk as RustCoreJwk
import web5.sdk.rust.JwkData as RustCoreJwkData

/**
 * Partial representation of a [JSON Web Key as per RFC7517](https://tools.ietf.org/html/rfc7517).
 * Note that this is a subset of the spec.
 */
data class Jwk (
    val alg: String? = null,
    val kty: String,
    val crv: String,
    val x: String,
    val y: String? = null,
    val d: String? = null
) {
    internal val rustCoreJwkData: RustCoreJwkData = RustCoreJwkData(
        alg,
        kty,
        crv,
        d,
        x,
        y
    )

    internal companion object {
        fun fromRustCoreJwkData(rustCoreJwkData: RustCoreJwkData): Jwk {
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

    fun computeThumbprint(): String {
        val rustCoreJwk = RustCoreJwk(rustCoreJwkData)
        return rustCoreJwk.computeThumbprint()
    }
}
