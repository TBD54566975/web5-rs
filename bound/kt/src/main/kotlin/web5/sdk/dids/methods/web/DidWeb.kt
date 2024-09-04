package web5.sdk.dids.methods.web

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.Dsa
import web5.sdk.crypto.dsaToRustCore
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.dids.Service
import web5.sdk.dids.VerificationMethod
import web5.sdk.rust.didWebCreate as rustCoreDidWebCreate
import web5.sdk.rust.didWebResolve as rustCoreDidWebResolve
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

data class DidWebCreateOptions(
    val keyManager: KeyManager? = null,
    val dsa: Dsa? = null,
    val service: List<Service>? = null,
    val controller: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val verificationMethod: List<VerificationMethod>? = null
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
            try {
                val rustCoreOptions = options?.let { opts ->
                    web5.sdk.rust.DidWebCreateOptions(
                        keyManager = opts.keyManager?.let { ToInnerKeyManager(it) },
                        dsa = opts.dsa?.let { dsaToRustCore(it) },
                        service = opts.service?.map { it.toRustCore() },
                        controller = opts.controller,
                        alsoKnownAs = opts.alsoKnownAs,
                        verificationMethod = opts.verificationMethod?.map { it.toRustCore() }
                    )
                }
                val rustCoreBearerDid = rustCoreDidWebCreate(domain, rustCoreOptions)
                return BearerDid.fromRustCoreBearerDid(rustCoreBearerDid)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            try {
                val rustCoreResolutionResult = rustCoreDidWebResolve(uri)
                return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }
}
