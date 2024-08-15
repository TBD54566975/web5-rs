package web5.sdk.vc

import com.fasterxml.jackson.annotation.JsonValue
import web5.sdk.Json
import java.time.Instant
import java.util.Date
import web5.sdk.rust.VerifiableCredential as RustCoreVerifiableCredential
import web5.sdk.rust.VerifiableCredentialCreateOptionsData as RustCoreVerifiableCredentialCreateOptions

data class VerifiableCredentialCreateOptions(
    val id: String? = null,
    var context: List<String>? = null,
    var type: List<String>? = null,
    var issuanceDate: Date? = null,
    var expirationDate: Date? = null
)

class VerifiableCredential private constructor(
    val context: List<String>,
    val type: List<String>,
    val id: String,
    val issuer: Issuer,
    val credentialSubject: CredentialSubject,
    val issuanceDate: Instant,
    val expirationDate: Instant? = null,
    internal val rustCoreVerifiableCredential: RustCoreVerifiableCredential,
) {
    companion object {
        fun create(
            issuer: Issuer,
            credentialSubject: CredentialSubject,
            options: VerifiableCredentialCreateOptions? = null): VerifiableCredential {

            val jsonSerializedIssuer = Json.stringify(issuer)
            val jsonSerializedCredentialSubject = Json.stringify(credentialSubject)

            val rustCoreVerifiableCredential = RustCoreVerifiableCredential.create(
                jsonSerializedIssuer,
                jsonSerializedCredentialSubject,
                RustCoreVerifiableCredentialCreateOptions(
                    options?.id,
                    options?.context,
                    options?.type,
                    options?.issuanceDate?.toInstant(),
                    options?.expirationDate?.toInstant(),
                )
            )

            val data = rustCoreVerifiableCredential.getData()

            return VerifiableCredential(
                data.context,
                data.type,
                data.id,
                issuer,
                credentialSubject,
                data.issuanceDate,
                data.expirationDate,
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