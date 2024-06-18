package web5.sdk.dids

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.crypto.keys.Jwk

import web5.sdk.rust.DocumentData as RustCoreDocumentData
import web5.sdk.rust.ServiceData as RustCoreServiceData
import web5.sdk.rust.VerificationMethodData as RustCoreVerificationMethodData

/**
 * Representation of a [DID Document](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md).
 *
 * @property id The DID URI for a particular DID subject is expressed using the id property in the DID document.
 * @property context The context property which can include multiple contexts as defined in the specification.
 * @property controller A DID controller is an entity that is authorized to make changes to a DID document.
 * @property alsoKnownAs The assertion that two or more DIDs (or other types of URI) refer to the same DID subject.
 * @property verificationMethod Cryptographic public keys, which can be used to authenticate or authorize interactions with the DID subject or associated parties.
 * @property authentication The authentication verification relationship used to specify how the DID subject is expected to be authenticated.
 * @property assertionMethod The assertionMethod verification relationship used to specify how the DID subject is expected to express claims.
 * @property keyAgreement The keyAgreement verification relationship used to specify how an entity can generate encryption material.
 * @property capabilityInvocation The capabilityInvocation verification relationship used to specify a verification method for invoking a cryptographic capability.
 * @property capabilityDelegation The capabilityDelegation verification relationship used to specify a mechanism for delegating a cryptographic capability.
 * @property service Services are used in DID documents to express ways of communicating with the DID subject or associated entities.
 */
data class Document(
    val id: String,
    @JsonProperty("@context")
    val context: List<String>? = null,
    val alsoKnownAs: List<String>? = null,
    val controller: List<String>? = null,
    val verificationMethod: List<VerificationMethod>? = null,
    val service: List<Service>? = null,
    val assertionMethod: List<String>? = null,
    val authentication: List<String>? = null,
    val keyAgreement: List<String>? = null,
    val capabilityDelegation: List<String>? = null,
    val capabilityInvocation: List<String>? = null
) {
    /**
     * Converts the DidDocument instance to a RustCoreDocumentData binding.
     *
     * @return RustCoreDocumentData the corresponding RustCoreDocumentData object.
     */
    fun toRustCore(): RustCoreDocumentData {
        return RustCoreDocumentData(
            id = this.id,
            context = this.context,
            controller = this.controller,
            alsoKnownAs = this.alsoKnownAs,
            verificationMethod = this.verificationMethod?.map { it.toRustCore() } ?: emptyList(),
            authentication = this.authentication,
            assertionMethod = this.assertionMethod,
            keyAgreement = this.keyAgreement,
            capabilityInvocation = this.capabilityInvocation,
            capabilityDelegation = this.capabilityDelegation,
            service = this.service?.map { it.toRustCore() } ?: emptyList()
        )
    }

    companion object {
        /**
         * Creates a DidDocument instance from a RustCoreDocumentData binding.
         *
         * @param documentData the RustCoreDocumentData object.
         * @return DidDocument the corresponding DidDocument instance.
         */
        fun fromRustCore(documentData: RustCoreDocumentData): Document {
            return Document(
                id = documentData.id,
                context = documentData.context,
                controller = documentData.controller,
                alsoKnownAs = documentData.alsoKnownAs,
                verificationMethod = documentData.verificationMethod.map { VerificationMethod.fromRustCore(it) },
                authentication = documentData.authentication,
                assertionMethod = documentData.assertionMethod,
                keyAgreement = documentData.keyAgreement,
                capabilityInvocation = documentData.capabilityInvocation,
                capabilityDelegation = documentData.capabilityDelegation,
                service = documentData.service?.map { Service.fromRustCore(it) }
            )
        }
    }
}

/**
 * A class representing a [Verification Method](https://www.w3.org/TR/did-core/#verification-methods).
 *
 * @property id The identifier for the verification method.
 * @property type The type of the verification method.
 * @property controller The controller of the verification method.
 * @property publicKeyJwk The public key in JWK format.
 */
class VerificationMethod(
    val id: String,
    val type: String,
    val controller: String,
    val publicKeyJwk: Jwk
) {
    /**
     * Converts the VerificationMethod instance to a RustCoreVerificationMethodData binding.
     *
     * @return RustCoreVerificationMethodData the corresponding RustCoreVerificationMethodData object.
     */
    fun toRustCore(): RustCoreVerificationMethodData {
        return RustCoreVerificationMethodData(
            id = this.id,
            type = this.type,
            controller = this.controller,
            publicKeyJwk = this.publicKeyJwk.toRustCore()
        )
    }

    companion object {
        /**
         * Creates a VerificationMethod instance from a RustCoreVerificationMethodData binding.
         *
         * @param data the RustCoreVerificationMethodData object.
         * @return VerificationMethod the corresponding VerificationMethod instance.
         */
        fun fromRustCore(data: RustCoreVerificationMethodData): VerificationMethod {
            return VerificationMethod(
                id = data.id,
                type = data.type,
                controller = data.controller,
                publicKeyJwk = Jwk.fromRustCore(data.publicKeyJwk)
            )
        }
    }
}

/**
 * Representation of a DID Document's [Service](https://www.w3.org/TR/did-core/#service).
 *
 * @property id The identifier for the service.
 * @property type The type of the service.
 * @property serviceEndpoint The endpoints of the service.
 */
class Service(
    val id: String,
    val type: String,
    val serviceEndpoint: List<String>
) {
    /**
     * Converts the Service instance to a RustCoreServiceData binding.
     *
     * @return RustCoreServiceData the corresponding RustCoreServiceData object.
     */
    fun toRustCore(): RustCoreServiceData {
        return RustCoreServiceData(
            id = this.id,
            type = this.type,
            serviceEndpoint = this.serviceEndpoint
        )
    }

    companion object {
        /**
         * Creates a Service instance from a RustCoreServiceData binding.
         *
         * @param data the RustCoreServiceData object.
         * @return Service the corresponding Service instance.
         */
        fun fromRustCore(data: RustCoreServiceData): Service {
            return Service(
                id = data.id,
                type = data.type,
                serviceEndpoint = data.serviceEndpoint
            )
        }
    }
}
