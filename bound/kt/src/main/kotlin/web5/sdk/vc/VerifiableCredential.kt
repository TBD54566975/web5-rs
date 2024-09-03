package web5.sdk.vc

import com.fasterxml.jackson.annotation.JsonValue
import web5.sdk.Json

import java.time.Instant
import java.util.*

import web5.sdk.rust.CredentialStatusData as RustCoreCredentialStatus
import web5.sdk.rust.VerifiableCredential as RustCoreVerifiableCredential
import web5.sdk.rust.VerifiableCredentialCreateOptionsData as RustCoreVerifiableCredentialCreateOptions

data class CredentialStatus(
    var id: String,
    var type: String,
    var statusPurpose: String,
    var statusListIndex: String,
    var statusListCredential: String
)

data class VerifiableCredentialCreateOptions(
    val id: String? = null,
    var context: List<String>? = null,
    var type: List<String>? = null,
    var issuanceDate: Date? = null,
    var expirationDate: Date? = null,
    var credentialStatus: CredentialStatus? = null
)

class VerifiableCredential private constructor(
    val context: List<String>,
    val type: List<String>,
    val id: String,
    val issuer: Issuer,
    val credentialSubject: CredentialSubject,
    val issuanceDate: Date,
    val expirationDate: Date? = null,
    internal val rustCoreVerifiableCredential: RustCoreVerifiableCredential,
) {
    companion object {
        fun create(
            issuer: Issuer,
            credentialSubject: CredentialSubject,
            options: VerifiableCredentialCreateOptions? = null): VerifiableCredential {

            val jsonSerializedIssuer = Json.stringify(issuer)
            val jsonSerializedCredentialSubject = Json.stringify(credentialSubject)

            val rustCoreCredentialStatus = options?.credentialStatus?.let {
                RustCoreCredentialStatus(
                    id = it.id,
                    type = it.type,
                    statusPurpose = it.statusPurpose,
                    statusListIndex = it.statusListIndex,
                    statusListCredential = it.statusListCredential
                )
            }

            val rustCoreVerifiableCredential = RustCoreVerifiableCredential.create(
                jsonSerializedIssuer,
                jsonSerializedCredentialSubject,
                RustCoreVerifiableCredentialCreateOptions(
                    options?.id,
                    options?.context,
                    options?.type,
                    options?.issuanceDate?.toInstant(),
                    options?.expirationDate?.toInstant(),
                    rustCoreCredentialStatus
                )
            )

            val data = rustCoreVerifiableCredential.getData()

            return VerifiableCredential(
                data.context,
                data.type,
                data.id,
                issuer,
                credentialSubject,
                Date.from(data.issuanceDate),
                data.expirationDate?.let { Date.from(it) },
                rustCoreVerifiableCredential,
            )
        }
    }
}

sealed class Issuer {
    data class StringIssuer(val value: String) : Issuer() {
        @JsonValue
        fun toJson(): String = value
    }

    data class ObjectIssuer(
        val id: String,
        val name: String,
        val additionalProperties: Map<String, Any> = emptyMap()
    ) : Issuer() {
        @JsonValue
        fun toJson(): Map<String, Any> {
            return mapOf("id" to id, "name" to name) + additionalProperties
        }
    }
}

data class CredentialSubject(
    val id: String,
    val additionalProperties: Map<String, Any> = emptyMap()
) {
    @JsonValue
    fun toJson(): Map<String, Any> {
        return mapOf(
            "id" to id,
        ) + additionalProperties
    }
}