package web5.sdk.dids.methods.web

import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.keys.ToOuterKeyManager
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.Dsa
import web5.sdk.rust.ServiceData
import web5.sdk.rust.VerificationMethodData
import web5.sdk.rust.didWebCreate as rustCoreDidWebCreate
import web5.sdk.rust.didWebResolve as rustCoreDidWebResolve

data class DidWebCreateOptions(
    val keyManager: KeyManager? = null,
    val dsa: Dsa? = null,
    val service: List<ServiceData>? = null,
    val controller: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val verificationMethod: List<VerificationMethodData>? = null
)

/**
 * A class representing a DID (Decentralized Identifier) using the Web method.
 */
class DidWeb {
    companion object {
        /**
         * Create a DidWeb BearerDid using available options.
         *
         * @param domain The domain for the given did:web.
         * @param options The set of options to configure creation.
         */
        fun create(domain: String, options: DidWebCreateOptions? = null): BearerDid {
            val rustCoreOptions = options?.let { opts ->
                web5.sdk.rust.DidWebCreateOptions(
                    keyManager = opts.keyManager?.let { ToInnerKeyManager(it) },
                    dsa = opts.dsa,
                    service = opts.service,
                    controller = opts.controller,
                    alsoKnownAs = opts.alsoKnownAs,
                    verificationMethod = opts.verificationMethod
                )
            }
            val rustCoreBearerDid = rustCoreDidWebCreate(domain, rustCoreOptions)
            return BearerDid(rustCoreBearerDid)
        }

        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            val rustCoreResolutionResult = rustCoreDidWebResolve(uri)
            return rustCoreResolutionResult.getData()
        }
    }
}
