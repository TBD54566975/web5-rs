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

/**
 * Represents the options used when creating a DID using the JWK (JSON Web Key) method.
 *
 * @property keyManager The key manager used for key storage and management.
 * @property dsa The digital signature algorithm (DSA) used for key generation (e.g., Ed25519, Secp256k1).
 */
data class DidJwkCreateOptions(
    val keyManager: KeyManager? = null,
    val dsa: Dsa? = null
)

/**
 * Provides functionality for creating and resolving "did:jwk" method Decentralized Identifiers (DIDs).
 *
 * The "did:jwk" DID method derives a DID directly from a public key using the JSON Web Key (JWK) format.
 * This allows the DID to be self-verifiable, without relying on a ledger or blockchain.
 */
class DidJwk {
    companion object {
        /**
         * Creates a new "did:jwk" DID using the provided options.
         *
         * This method generates a "did:jwk" DID by creating a key pair and constructing a DID document
         * based on the JWK format. The method-specific identifier is derived from the public key.
         *
         * @param options The options to configure the DID creation. If not provided, default options are used.
         * @return A [BearerDid] object representing the newly created "did:jwk" DID.
         * @throws Web5Exception If an error occurs during DID creation.
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
            }
        }

        /**
         * Resolves a "did:jwk" DID into a [ResolutionResult].
         *
         * This method resolves a DID URI by decoding the method-specific identifier (the public key)
         * and constructing a DID document based on the JWK format.
         *
         * @param uri The DID URI to resolve.
         * @return A [ResolutionResult] containing the DID document and related metadata.
         * @throws Web5Exception If an error occurs during resolution.
         */
        @JvmStatic
        fun resolve(uri: String): ResolutionResult {
            try {
                val rustCoreResolutionResult = rustCoreDidJwkResolve(uri)
                return ResolutionResult.fromRustCoreResolutionResult(rustCoreResolutionResult)
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }
}
