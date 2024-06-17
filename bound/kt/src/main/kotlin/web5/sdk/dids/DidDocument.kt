package web5.sdk.dids

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.crypto.keys.Jwk

import web5.sdk.rust.DocumentData as RustCoreDocumentData
import web5.sdk.rust.ServiceData as RustCoreServiceData
import web5.sdk.rust.VerificationMethodData as RustCoreVerificationMethodData

class VerificationMethod(
    val id: String,
    val type: String,
    val controller: String,
    val publicKeyJwk: Jwk? = null
) {
    fun toVerificationMethodData(): RustCoreVerificationMethodData {
        return RustCoreVerificationMethodData(
            id = this.id,
            type = this.type,
            controller = this.controller,
            publicKeyJwk = this.publicKeyJwk!!.toBinding()
        )
    }

    companion object {
        fun fromVerificationMethodData(data: RustCoreVerificationMethodData): VerificationMethod {
            return VerificationMethod(
                id = data.id,
                type = data.type,
                controller = data.controller,
                publicKeyJwk = Jwk.fromBinding(data.publicKeyJwk)
            )
        }
    }
}

class Service(
    val id: String,
    val type: String,
    val serviceEndpoint: List<String>
) {
    fun toServiceData(): RustCoreServiceData {
        return RustCoreServiceData(
            id = this.id,
            type = this.type,
            serviceEndpoint = this.serviceEndpoint
        )
    }

    companion object {
        fun fromServiceData(data: RustCoreServiceData): Service {
            return Service(
                id = data.id,
                type = data.type,
                serviceEndpoint = data.serviceEndpoint
            )
        }
    }
}

data class DidDocument(
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
    fun toBinding(): RustCoreDocumentData {
        return RustCoreDocumentData(
            id = this.id,
            context = this.context,
            controller = this.controller,
            alsoKnownAs = this.alsoKnownAs,
            verificationMethod = this.verificationMethod?.map { it.toVerificationMethodData() } ?: emptyList(),
            authentication = this.authentication,
            assertionMethod = this.assertionMethod,
            keyAgreement = this.keyAgreement,
            capabilityInvocation = this.capabilityInvocation,
            capabilityDelegation = this.capabilityDelegation,
            service = this.service?.map { it.toServiceData() } ?: emptyList()
        )
    }

    companion object {
        fun fromBinding(documentData: RustCoreDocumentData): DidDocument {
            return DidDocument(
                id = documentData.id,
                context = documentData.context,
                controller = documentData.controller,
                alsoKnownAs = documentData.alsoKnownAs,
                verificationMethod = documentData.verificationMethod?.map { VerificationMethod.fromVerificationMethodData(it) },
                authentication = documentData.authentication,
                assertionMethod = documentData.assertionMethod,
                keyAgreement = documentData.keyAgreement,
                capabilityInvocation = documentData.capabilityInvocation,
                capabilityDelegation = documentData.capabilityDelegation,
                service = documentData.service?.map { Service.fromServiceData(it) }
            )
        }
    }
}