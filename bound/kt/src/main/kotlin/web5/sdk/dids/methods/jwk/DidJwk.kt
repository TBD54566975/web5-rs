package web5.sdk.dids.methods.jwk

import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.Did
import web5.sdk.dids.Document
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.didJwkResolve as rustCoreDidJwkResolve
import web5.sdk.rust.DidJwk as RustCoreDidJwk

/**
 * A class representing a DID (Decentralized Identifier) using the JWK (JSON Web Key) method.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class DidJwk {
    val did: Did
    val document: Document

    /**
     * Constructs a DidJwk instance using a public key.
     *
     * @param publicKey The public key represented as a Jwk.
     */
    constructor(publicKey: Jwk) {
        val rustCoreDidJwk = RustCoreDidJwk.fromPublicJwk(publicKey.rustCoreJwkData)

        this.did = Did.fromRustCoreDidData(rustCoreDidJwk.getData().did)
        this.document = rustCoreDidJwk.getData().document
    }

    /**
     * Constructs a DidJwk instance using a DID URI.
     *
     * @param uri The DID URI.
     */
    constructor(uri: String) {
        val rustCoreDidJwk = RustCoreDidJwk.fromUri(uri)

        this.did = Did.fromRustCoreDidData(rustCoreDidJwk.getData().did)
        this.document = rustCoreDidJwk.getData().document
    }

    companion object {
        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            val rustCoreResolutionObject = rustCoreDidJwkResolve(uri).getData()
            return rustCoreResolutionObject
        }
    }
}
