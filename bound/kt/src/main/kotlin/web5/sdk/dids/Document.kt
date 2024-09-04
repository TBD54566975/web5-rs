package web5.sdk.dids

import web5.sdk.Web5Exception
import web5.sdk.crypto.keys.Jwk
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Representation of a [DID Document](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md).
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
