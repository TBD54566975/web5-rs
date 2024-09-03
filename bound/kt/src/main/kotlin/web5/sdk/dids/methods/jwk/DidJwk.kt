package web5.sdk.dids.methods.jwk

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.Dsa
import web5.sdk.crypto.dsaToRustCore
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.didJwkCreate
import web5.sdk.rust.DidJwkCreateOptions as RustCoreDidJwkCreateOptions
import web5.sdk.rust.didJwkResolve as rustCoreDidJwkResolve
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

data class DidJwkCreateOptions(
    val keyManager: KeyManager? = null,
    val dsa: Dsa? = null
)

/**
 * A class representing a DID (Decentralized Identifier) using the JWK (JSON Web Key) method.
 */
class DidJwk {
    companion object {
        /**
         * Create a DidJwk BearerDid using available options.
         *
         * @param options The set of options to configure creation.
         */
        fun create(options: DidJwkCreateOptions? = null): BearerDid {
            try {
                val rustCoreOptions = options?.let { opts ->
                    RustCoreDidJwkCreateOptions(
                        keyManager = opts.keyManager?.let { ToInnerKeyManager(it) },
                        dsa = opts.dsa?.let { dsaToRustCore(it) }
                    ) }
                val rustCoreBearerDid = didJwkCreate(rustCoreOptions)
                return BearerDid.fromRustCoreBearerDid(rustCoreBearerDid)
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
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            try {
                val rustCoreResolutionResult = rustCoreDidJwkResolve(uri)
                return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            } catch (e: Exception) {
                throw e
            }
        }
    }
}
