package web5.sdk.dids.did.methods

import com.fasterxml.jackson.annotation.JsonProperty
import web5.sdk.DocumentData
import web5.sdk.ServiceData
import web5.sdk.VerificationMethodData
import web5.sdk.crypto.keys.Jwk

public class VerificationMethod(
    public val id: String,
    public val type: String,
    public val controller: String,
    public val publicKeyJwk: Jwk? = null
) {
    fun toVerificationMethodData(): VerificationMethodData {
        return VerificationMethodData(
            id = this.id,
            type = this.type,
            controller = this.controller,
            publicKeyJwk = this.publicKeyJwk!!.toBinded()
        )
    }

    companion object {
        fun fromVerificationMethodData(data: VerificationMethodData): VerificationMethod {
            return VerificationMethod(
                id = data.id,
                type = data.type,
                controller = data.controller,
                publicKeyJwk = Jwk.fromBinded(data.publicKeyJwk)
            )
        }
    }
}

public class Service(
    public val id: String,
    public val type: String,
    public val serviceEndpoint: List<String>
) {
    fun toServiceData(): ServiceData {
        return ServiceData(
            id = this.id,
            type = this.type,
            serviceEndpoint = this.serviceEndpoint
        )
    }

    companion object {
        fun fromServiceData(data: ServiceData): Service {
            return Service(
                id = data.id,
                type = data.type,
                serviceEndpoint = data.serviceEndpoint
            )
        }
    }
}

// Define the DidDocument class
public data class DidDocument(
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
    fun toBinded(): DocumentData {
        return DocumentData(
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
        fun fromBinded(documentData: DocumentData): DidDocument {
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