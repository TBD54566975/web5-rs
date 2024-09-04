package web5.sdk.vc

import com.fasterxml.jackson.annotation.JsonAnyGetter
import com.fasterxml.jackson.annotation.JsonAnySetter
import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonValue
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.module.kotlin.readValue
import web5.sdk.Json
import web5.sdk.dids.BearerDid
import java.util.Date
import web5.sdk.rust.CredentialStatusData as RustCoreCredentialStatus
import web5.sdk.rust.VerifiableCredential as RustCoreVerifiableCredential
import web5.sdk.rust.VerifiableCredentialCreateOptionsData as RustCoreVerifiableCredentialCreateOptions
import web5.sdk.rust.CredentialSchemaData as RustCoreCredentialSchema

data class CredentialStatus(
    var id: String,
    var type: String,
    var statusPurpose: String,
    var statusListIndex: String,
    var statusListCredential: String
)

data class VerifiableCredentialCreateOptions(
    val id: String? = null,
    val context: List<String>? = null,
    val type: List<String>? = null,
    val issuanceDate: Date? = null,
    val expirationDate: Date? = null,
    var credentialStatus: CredentialStatus? = null,
    val credentialSchema: CredentialSchema? = null,
    val evidence: List<Map<String, Any>>? = null
)

data class VerifiableCredential private constructor(
    val context: List<String>,
    val type: List<String>,
    val id: String,
    val issuer: Issuer,
    val credentialSubject: CredentialSubject,
    val issuanceDate: Date,
    val expirationDate: Date? = null,
    val credentialStatus: CredentialStatus? = null,
    val credentialSchema: CredentialSchema? = null,
    val evidence: List<Map<String, Any>>? = null,
    internal val rustCoreVerifiableCredential: RustCoreVerifiableCredential,
) {
    companion object {
        internal fun fromRustCore(rustCoreVerifiableCredential: RustCoreVerifiableCredential): VerifiableCredential {
            val data = rustCoreVerifiableCredential.getData()

            val issuer = Json.jsonMapper.readValue(data.jsonSerializedIssuer, Issuer::class.java)
            val credentialSubject = Json.jsonMapper.readValue(data.jsonSerializedCredentialSubject, CredentialSubject::class.java)
            val evidence = data.jsonSerializedEvidence?.let { Json.jsonMapper.readValue<List<Map<String, Any>>>(it) }

            return VerifiableCredential(
                data.context,
                data.type,
                data.id,
                issuer,
                credentialSubject,
                Date.from(data.issuanceDate),
                data.expirationDate?.let { Date.from(it) },
                data.credentialStatus?.let { CredentialStatus(it.id, it.type, it.statusPurpose, it.statusListIndex, it.statusListCredential) },
                data.credentialSchema?.let { CredentialSchema(it.id, it.type) },
                evidence,
                rustCoreVerifiableCredential
            )
        }

        fun create(
            issuer: Issuer,
            credentialSubject: CredentialSubject,
            options: VerifiableCredentialCreateOptions? = null): VerifiableCredential {

            val jsonSerializedIssuer = Json.stringify(issuer)
            val jsonSerializedCredentialSubject = Json.stringify(credentialSubject)
            val jsonSerializedEvidence = options?.evidence?.let { Json.stringify(it) }

            val rustCoreVerifiableCredential = RustCoreVerifiableCredential.create(
                jsonSerializedIssuer,
                jsonSerializedCredentialSubject,
                RustCoreVerifiableCredentialCreateOptions(
                    options?.id,
                    options?.context,
                    options?.type,
                    options?.issuanceDate?.toInstant(),
                    options?.expirationDate?.toInstant(),
                    options?.credentialStatus?.let { RustCoreCredentialStatus(it.id, it.type, it.statusPurpose, it.statusListIndex, it.statusListCredential) },
                    options?.credentialSchema?.let { RustCoreCredentialSchema(it.id, it.type) },
                    jsonSerializedEvidence
                )
            )

            val data = rustCoreVerifiableCredential.getData()
            val evidence = data.jsonSerializedEvidence?.let { Json.jsonMapper.readValue<List<Map<String, Any>>>(it) }

            return VerifiableCredential(
                data.context,
                data.type,
                data.id,
                issuer,
                credentialSubject,
                Date.from(data.issuanceDate),
                data.expirationDate?.let { Date.from(it) },
                data.credentialStatus?.let { CredentialStatus(it.id, it.type, it.statusPurpose, it.statusListIndex, it.statusListCredential) },
                data.credentialSchema?.let { CredentialSchema(it.id, it.type) },
                evidence,
                rustCoreVerifiableCredential
            )
        }

        fun fromVcJwt(vcJwt: String, verify: Boolean): VerifiableCredential {
            val rustCoreVerifiableCredential = RustCoreVerifiableCredential.fromVcJwt(vcJwt, verify)
            val data = rustCoreVerifiableCredential.getData()

            val issuer = Json.jsonMapper.readValue(data.jsonSerializedIssuer, Issuer::class.java)
            val credentialSubject = Json.jsonMapper.readValue(data.jsonSerializedCredentialSubject, CredentialSubject::class.java)
            val evidence = data.jsonSerializedEvidence?.let { Json.jsonMapper.readValue<List<Map<String, Any>>>(it) }

            return VerifiableCredential(
                data.context,
                data.type,
                data.id,
                issuer,
                credentialSubject,
                Date.from(data.issuanceDate),
                data.expirationDate?.let { Date.from(it) },
                data.credentialStatus?.let { CredentialStatus(it.id, it.type, it.statusPurpose, it.statusListIndex, it.statusListCredential) },
                data.credentialSchema?.let { CredentialSchema(it.id, it.type) },
                evidence,
                rustCoreVerifiableCredential
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
    @JsonIgnore
    val additionalProperties: Map<String, Any> = mutableMapOf()
) {
    @JsonAnyGetter
    internal fun getAdditionalProperties(): Map<String, Any> {
        return additionalProperties
    }

    @JsonAnySetter
    internal fun setAdditionalProperty(key: String, value: Any) {
        (additionalProperties as MutableMap)[key] = value
    }

    @JsonValue
    fun toJson(): Map<String, Any> {
        return mapOf("id" to id) + additionalProperties
    }
}

data class CredentialSchema(
    val id: String,
    val type: String
)