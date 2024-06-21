package web5.sdk.dids

import web5.sdk.crypto.keys.KeyManagerInterface
import web5.sdk.crypto.signers.SignerInterface
import web5.sdk.rust.KeyManager

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

    private val rustCoreBearerDid: RustCoreBearerDid

    /**
     * Constructs a BearerDid instance using a DID URI and a key manager.
     *
     * @param uri The DID URI.
     * @param keyManager The key manager to handle keys.
     */
    constructor(uri: String, keyManager: KeyManagerInterface) {
        // TODO: This can never work
        val keyManagerImpl = keyManager as KeyManager
        this.rustCoreBearerDid = RustCoreBearerDid(uri, keyManagerImpl)
        this.did = this.rustCoreBearerDid.getData().did
        this.document = this.rustCoreBearerDid.getData().document
    }

    /**
     * Returns a signer for the DID.
     *
     * @return Signer The signer for the DID.
     */
    fun getSigner(): SignerInterface {
        val kid = document.verificationMethod[0].id
        return rustCoreBearerDid.getSigner(kid)
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
