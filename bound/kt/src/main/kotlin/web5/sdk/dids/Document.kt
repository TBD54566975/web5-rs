package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Representation of a [DID Document](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md).
 *
 * A `Document` represents the full DID document, which is a set of data describing a DID subject, including
 * public keys, authentication mechanisms, services, and more. This class provides functionality for converting
 * the document to and from JSON format, as well as converting it to and from Rust core data.
 *
 * @property id The DID URI representing the subject of the DID document.
 * @property context A list of URIs defining the schema version used in the document (optional).
 * @property controller A list of entities authorized to make changes to the DID document (optional).
 * @property alsoKnownAs A list of alternative identifiers for the DID subject (optional).
 * @property verificationMethod A list of cryptographic public keys for authentication and authorization.
 * @property authentication Methods for authenticating the DID subject (optional).
 * @property assertionMethod Methods for expressing claims, such as issuing Verifiable Credentials (optional).
 * @property keyAgreement Methods for establishing secure communication channels (optional).
 * @property capabilityInvocation Methods used by the DID subject to invoke cryptographic capabilities (optional).
 * @property capabilityDelegation Methods used by the DID subject to delegate cryptographic capabilities (optional).
 * @property service A list of services provided by the DID subject (optional).
 */
data class Document(
    val id: String,
    val context: List<String>?,
    val controller: List<String>?,
    val alsoKnownAs: List<String>?,
    val verificationMethod: List<VerificationMethod>,
    val authentication: List<String>?,
    val assertionMethod: List<String>?,
    val keyAgreement: List<String>?,
    val capabilityInvocation: List<String>?,
    val capabilityDelegation: List<String>?,
    val service: List<Service>?
) {
    companion object {
        /**
         * Converts a JSON string into a `Document` object.
         *
         * This method parses a JSON string and constructs a `Document` instance from the parsed data.
         * It uses Rust core functionality to handle the underlying parsing.
         *
         * @param json The JSON string representing the DID document.
         * @return A `Document` object.
         * @throws Web5Exception If parsing the JSON string fails.
         *
         * @example
         * ```
         * val jsonString = """{ "id": "did:example:123", "verificationMethod": [...] }"""
         * val document = Document.fromJsonString(jsonString)
         * println(document.id)  // Output: "did:example:123"
         * ```
         */
        fun fromJsonString(json: String): Document {
            try {
                return fromRustCore(web5.sdk.rust.Document.fromJsonString(json).getData())
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        internal fun fromRustCore(document: web5.sdk.rust.DocumentData): Document {
            return Document(
                document.id,
                document.context,
                document.controller,
                document.alsoKnownAs,
                document.verificationMethod.map { VerificationMethod.fromRustCore(it) },
                document.authentication,
                document.assertionMethod,
                document.keyAgreement,
                document.capabilityInvocation,
                document.capabilityDelegation,
                document.service?.map { Service.fromRustCore(it) }
            )
        }
    }

    /**
     * Converts the `Document` object to a JSON string.
     *
     * This method serializes the `Document` instance into a JSON string using Rust core functionality.
     *
     * @return The JSON string representing the DID document.
     * @throws Web5Exception If serialization to JSON fails.
     *
     * @example
     * ```
     * val document = Document(id = "did:example:123", verificationMethod = listOf(...))
     * val jsonString = document.toJsonString()
     * println(jsonString)  // Output: JSON representation of the document
     * ```
     */
    fun toJsonString(): String {
        try {
            return web5.sdk.rust.Document(toRustCore()).toJsonString()
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }

    internal fun toRustCore(): web5.sdk.rust.DocumentData {
        return web5.sdk.rust.DocumentData(
            id,
            context,
            controller,
            alsoKnownAs,
            verificationMethod.map { it.toRustCore() },
            authentication,
            assertionMethod,
            keyAgreement,
            capabilityInvocation,
            capabilityDelegation,
            service?.map { it.toRustCore() }
        )
    }
}

/**
 * Represents a verification method within a DID document.
 *
 * A `VerificationMethod` describes a cryptographic public key that can be used to authenticate or authorize
 * interactions with the DID subject.
 *
 * @property id The ID of the verification method.
 * @property type The type of verification method (e.g., "JsonWebKey").
 * @property controller The controller of the verification method.
 * @property publicKeyJwk The public key in JWK format.
 */
data class VerificationMethod(
    val id: String,
    val type: String,
    val controller: String,
    val publicKeyJwk: Jwk
) {
    companion object {
        internal fun fromRustCore(verificationMethod: web5.sdk.rust.VerificationMethodData): VerificationMethod {
            return VerificationMethod(
                verificationMethod.id,
                verificationMethod.type,
                verificationMethod.controller,
                Jwk.fromRustCoreJwkData(verificationMethod.publicKeyJwk)
            )
        }
    }

    internal fun toRustCore(): web5.sdk.rust.VerificationMethodData {
        return web5.sdk.rust.VerificationMethodData(
            id, type, controller, publicKeyJwk.rustCoreJwkData
        )
    }
}

/**
 * Represents a service within a DID document.
 *
 * A `Service` describes an endpoint that can be used to interact with the DID subject, such as
 * communication, discovery, or storage services.
 *
 * @property id The ID of the service.
 * @property type The type of service.
 * @property serviceEndpoint A list of endpoints where the service can be accessed.
 */
data class Service(
    val id: String,
    val type: String,
    val serviceEndpoint: List<String>
) {
    companion object {
        internal fun fromRustCore(service: web5.sdk.rust.ServiceData): Service {
            return Service(
                service.id,
                service.type,
                service.serviceEndpoint
            )
        }
    }

    internal fun toRustCore(): web5.sdk.rust.ServiceData {
        return web5.sdk.rust.ServiceData(id, type, serviceEndpoint)
    }
}
