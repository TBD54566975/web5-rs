package web5.sdk.dids.methods.web

import kotlinx.coroutines.runBlocking
import web5.sdk.crypto.keys.Jwk
import web5.sdk.dids.Did
import web5.sdk.dids.Document
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.didWebResolve as rustCoreDidWebResolve
import web5.sdk.rust.DidWeb as RustCoreDidWeb

/**
 * A class representing a DID (Decentralized Identifier) using the Web method.
 *
 * @property did The DID associated with this instance.
 * @property document The DID document associated with this instance.
 */
class DidWeb {
    val did: Did
    val document: Document

    /**
     * Constructs a DidWeb instance using a DID URI.
     *
     * @param uri The DID URI.
     */
    constructor(uri: String) {
        val rustCoreDidWeb = runBlocking {
            RustCoreDidWeb.fromUri(uri)
        }

        this.did = Did.fromRustCoreDidData(rustCoreDidWeb.getData().did)
        this.document = rustCoreDidWeb.getData().document
    }

    /**
     * Constructs a DidWeb instance using a domain and public key jwk
     *
     * @param domain The DID domain name.
     */
    constructor(domain: String, publicKey: Jwk) {
        val rustCoreDidWeb = runBlocking {
            RustCoreDidWeb.fromPublicJwk(domain, publicKey);
        }

        this.did = Did.fromRustCoreDidData(rustCoreDidWeb.getData().did)
        this.document = rustCoreDidWeb.getData().document
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
            val rustCoreResolutionObject =
                runBlocking {
                    rustCoreDidWebResolve(uri).getData()
                }
            return rustCoreResolutionObject
        }
    }
}
