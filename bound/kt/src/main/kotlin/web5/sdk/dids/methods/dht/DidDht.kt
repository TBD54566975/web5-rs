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
 * A class representing a DID (Decentralized Identifier) using the DHT method.
 */
class DidDht {
    companion object {
        /**
         * Create a DidDht BearerDid using available options.
         *
         * @param options The set of options to configure creation.
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
            } catch (e: Exception) {
                throw e
            }
        }

        /**
         * Publish a DidDht BearerDid using available options.
         *
         * @param bearerDid The DidDht BearerDid instance to publish.
         * @param gatewayUrl The optional gateway URL to publish to.
         */
        fun publish(bearerDid: BearerDid, gatewayUrl: String? = null) {
            try {
                web5.sdk.rust.didDhtPublish(bearerDid.rustCoreBearerDid, gatewayUrl)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            } catch (e: Exception) {
                throw e
            }
        }

        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @param gatewayUrl The optional gateway URL to resolve from.
         */
        @JvmStatic
        fun resolve(uri: String, gatewayUrl: String? = null): ResolutionResult {
            try {
                val rustCoreResolutionResult = rustCoreDidDhtResolve(uri, gatewayUrl)
                return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            } catch (e: Exception) {
                throw e
            }
        }
    }
}
