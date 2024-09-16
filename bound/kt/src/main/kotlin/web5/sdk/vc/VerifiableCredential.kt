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
import java.util.Date
import web5.sdk.rust.CredentialStatusData as RustCoreCredentialStatus
import web5.sdk.Json
import web5.sdk.Web5Exception
import web5.sdk.dids.BearerDid
import web5.sdk.rust.VerifiableCredential as RustCoreVerifiableCredential
import web5.sdk.rust.VerifiableCredentialCreateOptionsData as RustCoreVerifiableCredentialCreateOptions
import web5.sdk.rust.CredentialSchemaData as RustCoreCredentialSchema
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents the status information of a Verifiable Credential.
 * CredentialStatus is used to indicate the revocation or suspension status of a credential.
 *
 * @property id The unique identifier for the credential status.
 * @property type The type(s) of the credential status.
 * @property statusPurpose The purpose of the status (e.g., "revocation" or "suspension").
 * @property statusListIndex The index in the status list indicating the credential's position.
 * @property statusListCredential The unique identifier for the Verifiable Credential that lists the status of the credential.
 */
data class CredentialStatus(
    var id: String,
    var type: String,
    var statusPurpose: String,
    var statusListIndex: String,
    var statusListCredential: String
)

/**
 * Represents the options available when creating a Verifiable Credential.
 * These options allow customization of various attributes of the credential during its creation.
 *
 * @property id The unique identifier for the Verifiable Credential. Optional. Defaults to `urn:uuid:{uuid}` if not provided.
 * @property context The context(s) defining the meaning of terms within the credential.
 * Defaults to `https://www.w3.org/2018/credentials/v1` if not provided.
 * @property type The type(s) of the Verifiable Credential, where "VerifiableCredential" is always included as the base type.
 * @property issuanceDate The issuance date of the credential. Defaults to the current date and time if not provided.
 * @property expirationDate The optional expiration date of the credential.
 * @property credentialStatus Optional status information of the credential (e.g., revocation or suspension).
 * @property credentialSchema Optional schema used to validate the credential's data structure.
 * @property evidence Optional array of evidence supporting the claims made in the credential.
 */
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

/**
 * Represents a Verifiable Credential according to the W3C Verifiable Credentials Data Model v1.1
 * and conformant to the Web5 specification.
 * A Verifiable Credential is a tamper-evident credential that has authorship that can be cryptographically verified.
 *
 * @property context A list of contexts used to define the semantic meaning of the data contained in the Verifiable Credential.
 * @property type The type(s) of the Verifiable Credential.
 * @property id The unique identifier for the Verifiable Credential.
 * @property issuer The entity (either a string or an object) that issued the credential.
 * @property credentialSubject The subject of the credential, containing claims about the entity described by the credential.
 * @property issuanceDate The date and time when the credential was issued.
 * @property expirationDate The optional expiration date after which the credential is no longer valid.
 * @property credentialStatus The credential status information, if applicable (e.g., revoked or suspended).
 * @property credentialSchema The schema used to validate the data structure of the credential.
 * @property evidence An array of evidence supporting the claims made in the credential.
 */
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

        /**
         * Creates a new Verifiable Credential with the specified issuer, subject, and optional creation options.
         *
         * @param issuer The entity issuing the credential. The `issuer` must be a valid DID.
         * @param credentialSubject The subject of the credential containing claims. The subject must be a valid DID.
         * @param options Optional parameters for creating the credential, such as schema or status.
         * @return The newly created Verifiable Credential.
         * @throws Web5Exception if the creation fails due to validation or other errors.
         *
         * Example usage:
         * ```
         * val issuerBearerDid = DidJwk.create()
         * val subjectDidUri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy"
         * val verifiableCredential = VerifiableCredential.create(
         *     Issuer.StringIssuer(issuerBearerDid.did.uri),
         *     CredentialSubject(id = subjectDidUri),
         *         )
         *     )
         * )
         * ```
         */
        fun create(
            issuer: Issuer,
            credentialSubject: CredentialSubject,
            options: VerifiableCredentialCreateOptions? = null): VerifiableCredential {
            try {
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
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        /**
         * Constructs a Verifiable Credential from a VC JWT (JSON Web Token).
         *
         * @param vcJwt The Verifiable Credential in JWT format, serialized as a compact JWS.
         * @param verify If true, verifies the integrity of the JWT by performing cryptographic verification against the signature, validating the VC Data Model, and validating the JSON Schema if present.
         * @return The deserialized and validated Verifiable Credential.
         * @throws Web5Exception if the JWT is invalid or verification fails.
         *
         * Example usage:
         * ```
         * val vcJwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw"
         * val verifiableCredential = VerifiableCredential.fromVcJwt(vcJwt, true)
         * ```
         */
        fun fromVcJwt(vcJwt: String, verify: Boolean): VerifiableCredential {
            try {
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
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }

    /**
     * Signs the Verifiable Credential using the specified Bearer DID and optional verification method.
     *
     * @param bearerDid The DID used to sign the credential.
     * @param verificationMethodId Optional identifier of the Verification Method for which to sign with.
     * @return A string representing the signed JWT, serialized as a compact JWS, of the Verifiable Credential.
     * @throws Web5Exception if the signing process fails.
     *
     * Example usage:
     * ```
     * val issuerBearerDid = DidJwk.create()
     * val subjectDidUri = "did:dht:ng4hmqtrgujox4agpf8okxihnyy1zqnq97qfeq15x8oar7yepzhy"
     * val verifiableCredential = VerifiableCredential.create(
     *     Issuer.StringIssuer(issuerBearerDid.did.uri),
     *     CredentialSubject(id = subjectDidUri),
     *     VerifiableCredentialCreateOptions(
     *         credentialStatus = CredentialStatus(
     *             id = "https://example.com/status/1",
     *             type = "StatusList2021Entry",
     *             statusPurpose = "revocation",
     *             statusListIndex = "3",
     *             statusListCredential = "https://example.com/status/1"
     *         )
     *     )
     * )
     *
     * val vcJwt = verifiableCredential.sign(issuerBearerDid)
     * ```
     */
    fun sign(bearerDid: BearerDid, verificationMethodId: String? = null): String {
        try {
            return rustCoreVerifiableCredential.sign(bearerDid.rustCoreBearerDid, verificationMethodId)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}

/**
 * Represents the issuer of the Verifiable Credential. It can be a string identifier or an object with additional properties.
 */
@JsonDeserialize(using = IssuerDeserializer::class)
sealed class Issuer {
    /**
     * Represents an issuer identified by a string (e.g., a DID or URL).
     */
    data class StringIssuer(val value: String) : Issuer() {
        @JsonValue
        fun toJson(): String = value
    }

    /**
     * Represents an issuer as an object, containing an ID, name, and additional properties.
     */
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

/**
 * Represents the subject of the Verifiable Credential, containing claims about the entity being described by the credential.
 *
 * @property id The identifier of the credential subject.
 * @property additionalProperties Additional properties associated with the credential subject.
 */
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

/**
 * Represents the credential schema, used to validate the data structure of the credential.
 *
 * @property id The unique identifier of the schema.
 * @property type The type of schema used for validation.
 */
data class CredentialSchema(
    val id: String,
    val type: String
)