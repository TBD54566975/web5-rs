package web5.sdk.vc

import com.fasterxml.jackson.module.kotlin.readValue
import web5.sdk.Json
import java.util.Date
import web5.sdk.Web5Exception
import web5.sdk.dids.BearerDid
import web5.sdk.rust.VerifiablePresentation as RustCoreVerifiablePresentation
import web5.sdk.rust.VerifiablePresentationCreateOptionsData as RustCoreVerifiablePresentationCreateOptions
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

data class VerifiablePresentationCreateOptions(
    val id: String? = null,
    val context: List<String>? = null,
    val type: List<String>? = null,
    val issuanceDate: Date? = null,
    val expirationDate: Date? = null,
    val additionalProperties: Map<String, Any>? = null
)

data class VerifiablePresentation private constructor(
    val context: List<String>,
    val type: List<String>,
    val id: String,
    val holder: String,
    val issuanceDate: Date,
    val expirationDate: Date? = null,
    val verifiableCredential: List<String>,
    val additionalProperties: Map<String, Any>?,
    internal val rustCoreVerifiablePresentation: RustCoreVerifiablePresentation,
) {
    companion object {
        fun create(
            holder: String,
            verifiableCredential: List<String>,
            options: VerifiablePresentationCreateOptions? = null
        ): VerifiablePresentation {
            try {
                val jsonSerializedAdditionalProperties = options?.additionalProperties?.let { Json.stringify(it) }

                val rustCoreVerifiablePresentation = RustCoreVerifiablePresentation.create(
                    holder,
                    verifiableCredential,
                    RustCoreVerifiablePresentationCreateOptions(
                        options?.id,
                        options?.context,
                        options?.type,
                        options?.issuanceDate?.toInstant(),
                        options?.expirationDate?.toInstant(),
                        jsonSerializedAdditionalProperties
                    )
                )

                val data = rustCoreVerifiablePresentation.getData()

                return VerifiablePresentation(
                    data.context,
                    data.type,
                    data.id,
                    holder,
                    Date.from(data.issuanceDate),
                    data.expirationDate?.let { Date.from(it) },
                    data.verifiableCredential,
                    options?.additionalProperties,
                    rustCoreVerifiablePresentation,
                )
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }

        fun fromVpJwt(vpJwt: String, verify: Boolean): VerifiablePresentation {
            try {
                val rustCoreVerifiablePresentation = RustCoreVerifiablePresentation.fromVpJwt(vpJwt, verify)
                val data = rustCoreVerifiablePresentation.getData()

                val additionalProperties = data.jsonSerializedAdditionalData?.let { Json.jsonMapper.readValue<Map<String, Any>>(it) }

                return VerifiablePresentation(
                    data.context,
                    data.type,
                    data.id,
                    data.holder,
                    Date.from(data.issuanceDate),
                    data.expirationDate?.let { Date.from(it) },
                    data.verifiableCredential,
                    additionalProperties,
                    rustCoreVerifiablePresentation
                )
            } catch (e: RustCoreException) {
                throw Web5Exception.fromRustCore(e)
            }
        }
    }

    fun sign(bearerDid: BearerDid, verificationMethodId: String? = null): String {
        try {
            return rustCoreVerifiablePresentation.sign(bearerDid.rustCoreBearerDid, verificationMethodId)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
