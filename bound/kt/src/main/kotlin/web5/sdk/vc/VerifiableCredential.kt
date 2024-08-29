package web5.sdk.vc

import com.fasterxml.jackson.annotation.JsonValue
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import web5.sdk.Json
import web5.sdk.dids.BearerDid
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
                Date.from(data.issuanceDate),
                data.expirationDate?.let { Date.from(it) },
                rustCoreVerifiableCredential,
            )
        }

        fun fromVcJwt(vcJwt: String, verify: Boolean): VerifiableCredential {
            val rustCoreVerifiableCredential = RustCoreVerifiableCredential.fromVcJwt(vcJwt, verify)
            val data = rustCoreVerifiableCredential.getData()

            val issuer = Json.jsonMapper.readValue(data.jsonSerializedIssuer, Issuer::class.java)
            val credentialSubject = Json.jsonMapper.readValue(data.jsonSerializedCredentialSubject, CredentialSubject::class.java)

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

    fun sign(bearerDid: BearerDid, verificationMethodId: String? = null): String {
        return rustCoreVerifiableCredential.sign(bearerDid.rustCoreBearerDid, verificationMethodId)
    }
}

@JsonDeserialize(using = IssuerDeserializer::class)
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

class IssuerDeserializer : JsonDeserializer<Issuer>() {
    override fun deserialize(p: JsonParser, ctxt: DeserializationContext): Issuer {
        val node = p.codec.readTree<JsonNode>(p)
        return if (node.isTextual) {
            Issuer.StringIssuer(node.asText())
        } else {
            val id = node.get("id").asText()
            val name = node.get("name").asText()
            val additionalProperties = mutableMapOf<String, Any>()
            node.fields().forEachRemaining { (key, value) ->
                if (key != "id" && key != "name") {
                    additionalProperties[key] = value
                }
            }
            Issuer.ObjectIssuer(id, name, additionalProperties)
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