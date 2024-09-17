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

/**
 * Represents the options used when creating a DID using the Web method.
 *
 * @property keyManager The key manager used for key storage and management.
 * @property dsa The digital signature algorithm (DSA) used for key generation (e.g., Ed25519, Secp256k1).
 * @property service A list of services to add to the DID document.
 * @property controller A list of controllers for the DID document.
 * @property alsoKnownAs A list of additional identifiers for the DID document.
 * @property verificationMethod A list of cryptographic verification methods for the DID document.
 */
data class DidWebCreateOptions(
    val keyManager: KeyManager? = null,
    val dsa: Dsa? = null,
    val service: List<Service>? = null,
    val controller: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val verificationMethod: List<VerificationMethod>? = null
)

/**
 * Provides functionality for creating and resolving "did:web" method Decentralized Identifiers (DIDs).
 *
 * The "did:web" method uses web domains as the identifier, allowing DIDs to be hosted on web servers
 * and leveraging the web's existing reputation system for decentralized identifiers.
 */
class DidWeb {
    companion object {
        /**
         * Creates a new "did:web" DID for a given domain using the provided options.
         *
         * This method generates a "did:web" DID by creating a key pair, constructing a DID document,
         * and associating it with a web domain. The DID can include services, verification methods, and controllers.
         *
         * @param domain The domain for which to create the "did:web" DID.
         * @param options The options to configure the DID creation. If not provided, default options are used.
         * @return A [BearerDid] object representing the newly created "did:web" DID.
         * @throws Web5Exception If an error occurs during DID creation.
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
         * Resolves a "did:web" DID into a [ResolutionResult].
         *
         * This method resolves a DID URI by fetching the DID document from the associated web domain.
         *
         * @param uri The DID URI to resolve.
         * @return A [ResolutionResult] containing the DID document and related metadata.
         * @throws Web5Exception If an error occurs during resolution.
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
