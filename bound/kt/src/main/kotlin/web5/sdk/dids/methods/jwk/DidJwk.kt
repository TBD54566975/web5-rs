package web5.sdk.dids.methods.jwk

import web5.sdk.crypto.keys.KeyManager
import web5.sdk.crypto.keys.ToInnerKeyManager
import web5.sdk.crypto.keys.ToOuterKeyManager
import web5.sdk.dids.BearerDid
import web5.sdk.dids.ResolutionResult
import web5.sdk.rust.Dsa
import web5.sdk.rust.didJwkCreate
import web5.sdk.rust.DidJwkCreateOptions as RustCoreDidJwkCreateOptions
import web5.sdk.rust.didJwkResolve as rustCoreDidJwkResolve

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
            val rustCoreOptions = options?.let { opts ->
                RustCoreDidJwkCreateOptions(
                keyManager = opts.keyManager?.let { ToInnerKeyManager(it) },
                dsa = opts.dsa
            ) }
            val rustCoreBearerDid = didJwkCreate(rustCoreOptions)
            val rustCoreBearerDidData = rustCoreBearerDid.getData()
            val keyManager = ToOuterKeyManager(rustCoreBearerDidData.keyManager)
            return BearerDid(rustCoreBearerDidData.did.uri, keyManager)
        }

        /**
         * Resolves a DID URI to a DidResolutionResult.
         *
         * @param uri The DID URI to resolve.
         * @return DidResolutionResult The result of the DID resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            val rustCoreResolutionObject = rustCoreDidJwkResolve(uri).getData()
            return rustCoreResolutionObject
        }
    }
}
