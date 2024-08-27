package web5.sdk.dids.methods.dht

import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.ServiceData
import web5.sdk.rust.VerificationMethodData
import web5.sdk.rust.didDhtResolve as rustCoreDidDhtResolve

data class DidDhtCreateOptions(
    val publish: Boolean? = true,
    val gatewayUrl: String? = null,
    val keyManager: KeyManager? = null,
    val service: List<ServiceData>? = null,
    val controller: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val verificationMethod: List<VerificationMethodData>? = null
)

data class DidDhtPublishOptions(
    val gatewayUrl: String? = null
)

data class DidDhtResolveOptions(
    val gatewayUrl: String? = null
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
            val rustCoreOptions = options?.let { opts ->
                web5.sdk.rust.DidDhtCreateOptions(
                    opts.publish,
                    opts.gatewayUrl,
                    opts.keyManager?.let { ToInnerKeyManager(it) },
                    opts.service,
                    opts.controller,
                    opts.alsoKnownAs,
                    opts.verificationMethod
                )
            }
            val rustCoreBearerDid = web5.sdk.rust.didDhtCreate(rustCoreOptions)
            return BearerDid.fromRustCoreBearerDid(rustCoreBearerDid)
        }

        /**
         * Publish a DidDht BearerDid using available options.
         *
         * @param bearerDid The DidDht BearerDid instance to publish.
         * @param options The set of options to configure publish.
         */
        fun publish(bearerDid: BearerDid, gatewayUrl: String? = null) {
           web5.sdk.rust.didDhtPublish(bearerDid.rustCoreBearerDid, gatewayUrl)
        }

        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String, gatewayUrl: String? = null): ResolutionResult {
            val rustCoreResolutionResult = rustCoreDidDhtResolve(uri, gatewayUrl)
            return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
        }
    }
}
