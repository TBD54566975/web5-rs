package web5.sdk.dids

import web5.sdk.crypto.signers.Signer
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.signers.OuterSigner

import web5.sdk.rust.BearerDid as RustCoreBearerDid

import web5.sdk.rust.didJwkResolve as rustCoreDidJwkResolve
import web5.sdk.rust.didWebResolve as rustCoreDidWebResolve
import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve

/**
 * Represents a Decentralized Identifier (DID) along with its DID document, key manager, metadata,
 * and convenience functions.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class BearerDid {
    val did: Did
    val document: Document
    val keyManager: KeyManager

    private val rustCoreBearerDid: RustCoreBearerDid

    /**
     * Constructs a BearerDid instance using a DID URI and a key manager.
     *
     * @param uri The DID URI.
     * @param keyManager The key manager to handle keys.
     */
    constructor(uri: String, keyManager: KeyManager) {
        this.rustCoreBearerDid = RustCoreBearerDid(uri, keyManager.getRustCoreKeyManager())

        this.did = this.rustCoreBearerDid.getData().did
        this.document = this.rustCoreBearerDid.getData().document
        this.keyManager = keyManager
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): Signer {
        val keyId = this.document.verificationMethod.first().id
        val innerSigner = this.rustCoreBearerDid.getSigner(keyId)
        return OuterSigner(innerSigner)
    }

    companion object {
        /**
         * Resolves a DID URI to a ResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return ResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        suspend fun resolve(uri: String): ResolutionResult {
            // TODO: Add a concept of a universal resolver - https://github.com/TBD54566975/web5-rs/issues/246
            val method = uri.substringAfter(":").substringBefore(":")
            if(method == "jwk") {
                return rustCoreDidJwkResolve(uri).getData()
            }

            if(method == "dht") {
                return rustCoreDidDhtResolve(uri).getData()
            }

            if(method == "web") {
                return rustCoreDidWebResolve(uri).getData()
            }

            throw IllegalArgumentException("Unknown method '$method'")
        }
    }
}
