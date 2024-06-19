package web5.sdk.dids.methods.dht

import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Signer
import web5.sdk.dids.Did
import web5.sdk.dids.Document
import web5.sdk.dids.ResolutionResult

import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve
import web5.sdk.rust.DidDht as RustCoreDidDht

/**
 * A class representing a DID (Decentralized Identifier) using the DHT method.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class DidDht {
    val did: Did
    val document: Document

    private val rustCoreDidDht: RustCoreDidDht

    /**
     * Constructs a DidDht instance using an identity key.
     *
     * @param identityKey The identity key represented as a Jwk.
     */
    constructor(identityKey: Jwk) {
        rustCoreDidDht = RustCoreDidDht.fromIdentityKey(identityKey)

        this.did = Did.fromRustCore(rustCoreDidDht.getData().did)
        this.document = rustCoreDidDht.getData().document
    }

    /**
     * Constructs a DidDht instance using a DID URI.
     *
     * @param uri The DID URI.
     */
    constructor(uri: String) {
        rustCoreDidDht = RustCoreDidDht.fromUri(uri)

        this.did = Did.fromRustCore(rustCoreDidDht.getData().did)
        this.document = rustCoreDidDht.getData().document
    }

    /**
     * Publishes the DID document.
     *
     * @param signer The signer used to sign the publish operation.
     */
    fun publish(signer: Signer) {
        // TODO: Implement publish method
    }

    /**
     * Deactivates the DID document.
     *
     * @param signer The signer used to sign the deactivate operation.
     */
    fun deactivate(signer: Signer) {
        // TODO: Implement deactivate method
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
            return rustCoreDidDhtResolve(uri).getData()
        }
    }
}
