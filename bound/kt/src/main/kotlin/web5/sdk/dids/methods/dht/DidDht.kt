package web5.sdk.dids.methods.dht

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.dids.Service
import web5.sdk.dids.VerificationMethod
import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents the options used when creating a DID using the DHT method.
 *
 * @property publish Specifies whether to publish the DID after creation (defaults to true).
 * @property gatewayUrl The URL of the gateway to use for publishing or resolving the DID.
 * @property keyManager The key manager used for key storage and management.
 * @property service A list of services to add to the DID document.
 * @property controller A list of controllers for the DID document.
 * @property alsoKnownAs A list of additional identifiers for the DID document.
 * @property verificationMethod A list of cryptographic verification methods for the DID document.
 */
data class DidDhtCreateOptions(
    val publish: Boolean? = true,
    val gatewayUrl: String? = null,
    val keyManager: KeyManager? = null,
    val service: List<Service>? = null,
    val controller: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val verificationMethod: List<VerificationMethod>? = null
)

/**
 * Provides functionality for creating, publishing, and resolving "did:dht" method Decentralized Identifiers (DIDs).
 *
 * The "did:dht" method derives a DID from an identity key and stores it on a Distributed Hash Table (DHT). The method-specific identifier
 * for "did:dht" is a z-base-32 encoded public key.
 */
class DidDht {
    companion object {
        /**
         * Creates a new "did:dht" DID using the provided options.
         *
         * This method generates a new "did:dht" DID by creating a key pair, constructing a DID document, and optionally publishing it
         * to the DHT.
         *
         * @param options The options to configure the DID creation. If not provided, default options are used.
         * @return A [BearerDid] object representing the newly created "did:dht" DID.
         * @throws Web5Exception If an error occurs during DID creation.
         */
        fun create(options: DidDhtCreateOptions? = null): BearerDid {
            try {
                val rustCoreOptions = options?.let { opts ->
                    web5.sdk.rust.DidDhtCreateOptions(
                        opts.publish,
                        opts.gatewayUrl,
                        opts.keyManager?.let { ToInnerKeyManager(it) },
                        opts.service?.map { it.toRustCore() },
                        opts.controller,
                        opts.alsoKnownAs,
                        opts.verificationMethod?.map { it.toRustCore() }
                    )
                }
                val rustCoreBearerDid = web5.sdk.rust.didDhtCreate(rustCoreOptions)
                return BearerDid.fromRustCoreBearerDid(rustCoreBearerDid)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        /**
         * Publishes a "did:dht" DID to the DHT.
         *
         * This method publishes the specified [BearerDid] to the DHT by converting its document into a packet, signing it,
         * and sending it to the DHT gateway.
         *
         * @param bearerDid The [BearerDid] instance representing the DID to publish.
         * @param gatewayUrl The optional gateway URL to use for publishing. If not provided, the default gateway is used.
         * @throws Web5Exception If an error occurs during publishing.
         */
        fun publish(bearerDid: BearerDid, gatewayUrl: String? = null) {
            try {
                web5.sdk.rust.didDhtPublish(bearerDid.rustCoreBearerDid, gatewayUrl)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        /**
         * Resolves a "did:dht" DID into a [ResolutionResult].
         *
         * This method resolves a DID URI by contacting a DHT gateway, retrieving the associated DID document, and verifying
         * its authenticity.
         *
         * @param uri The DID URI to resolve.
         * @param gatewayUrl The optional gateway URL to use for resolution. If not provided, the default gateway is used.
         * @return A [ResolutionResult] containing the DID document and related metadata.
         * @throws Web5Exception If an error occurs during resolution.
         */
        @JvmStatic
        fun resolve(uri: String, gatewayUrl: String? = null): ResolutionResult {
            try {
                val rustCoreResolutionResult = rustCoreDidDhtResolve(uri, gatewayUrl)
                return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }
}
