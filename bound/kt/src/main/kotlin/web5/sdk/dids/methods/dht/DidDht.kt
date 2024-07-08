package web5.sdk.dids.methods.dht

import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Signer
import web5.sdk.dids.Did
import web5.sdk.dids.Document
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.SystemTarget

import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve
import web5.sdk.rust.DidDht as RustCoreDidDht
import web5.sdk.rust.Signer as RustCoreSigner

/**
 * A class representing a DID (Decentralized Identifier) using the DHT method.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class DidDht {
    init {
        SystemTarget.set() // ensure the sys arch is set for first-time loading
    }

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

        this.did = rustCoreDidDht.getData().did
        this.document = rustCoreDidDht.getData().document
    }

    /**
     * Constructs a DidDht instance using a DID URI.
     *
     * @param uri The DID URI.
     */
    constructor(uri: String) {
        rustCoreDidDht = RustCoreDidDht.fromUri(uri)

        this.did = rustCoreDidDht.getData().did
        this.document = rustCoreDidDht.getData().document
    }

    /**
     * Publishes the DID document.
     *
     * @param signer The signer used to sign the publish operation.
     */
    fun publish(signer: Signer) {
        rustCoreDidDht.publish(signer as RustCoreSigner)
    }

    /**
     * Deactivates the DID document.
     *
     * @param signer The signer used to sign the deactivate operation.
     */
    fun deactivate(signer: Signer) {
        rustCoreDidDht.deactivate(signer as RustCoreSigner)
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
