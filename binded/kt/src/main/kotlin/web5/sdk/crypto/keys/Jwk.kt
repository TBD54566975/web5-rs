package web5.sdk.crypto.keys

import web5.sdk.JwkData

/**
 * Partial representation of a [JSON Web Key as per RFC7517](https://tools.ietf.org/html/rfc7517).
 * Note that this is a subset of the spec.
 */
data class Jwk(
    /**
     * Identifies the algorithm intended for use with the key.
     */
    val alg: String,

    /**
     * Represents the key type. e.g. EC for elliptic curve, OKP for Edwards curve
     */
    val kty: String,

    /**
     * Curve name for Elliptic Curve (EC) and Edwards Curve (Ed) keys.
     * e.g. secp256k1, Ed25519
     */
    val crv: String,

    /**
     * Private key component for EC or OKP keys.
     */
    val d: String? = null,

    /**
     * X coordinate for EC keys, or the public key for OKP.
     */
    val x: String,

    /**
     * Y coordinate for EC keys.
     */
    val y: String? = null,
) {

    /**
     * Converts this Jwk instance to a JwkData object.
     */
    fun toBinded(): JwkData {
        return JwkData(
            alg = this.alg,
            kty = this.kty,
            crv = this.crv,
            d = this.d,
            x = this.x,
            y = this.y
        )
    }

    companion object {
        /**
         * Creates an instance of Jwk from a JwkData object.
         */
        fun fromBinded(jwkData: JwkData): Jwk {
            return Jwk(
                alg = jwkData.alg,
                kty = jwkData.kty,
                crv = jwkData.crv,
                d = jwkData.d,
                x = jwkData.x,
                y = jwkData.y
            )
        }
    }
}


