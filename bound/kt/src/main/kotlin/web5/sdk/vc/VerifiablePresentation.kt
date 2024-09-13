package web5.sdk.vc

import com.fasterxml.jackson.module.kotlin.readValue
import web5.sdk.Json
import java.util.Date
import web5.sdk.Web5Exception
import web5.sdk.dids.BearerDid
import web5.sdk.rust.VerifiablePresentation as RustCoreVerifiablePresentation
import web5.sdk.rust.VerifiablePresentationCreateOptionsData as RustCoreVerifiablePresentationCreateOptions
import web5.sdk.rust.Web5Exception.Exception as RustCoreException

/**
 * Represents the options available when creating a Verifiable Presentation.
 * These options allow customization of various attributes of the presentation during its creation.
 *
 * @property id The unique identifier for the Verifiable Presentation. This is optional. Defaults to `urn:uuid:{uuid}` if not provided.
 * @property context The context(s) defining the meaning of terms within the presentation. The base context `https://www.w3.org/2018/credentials/v1` is always included.
 * @property type The type(s) of the Verifiable Presentation, where "VerifiablePresentation" is always included as the base type.
 * @property issuanceDate The issuance date of the presentation. Defaults to the current date and time if not provided.
 * @property expirationDate The optional expiration date for the presentation.
 * @property additionalProperties Additional data included in the presentation, represented as a key-value map.
 */
data class VerifiablePresentationCreateOptions(
    val id: String? = null,
    val context: List<String>? = null,
    val type: List<String>? = null,
    val issuanceDate: Date? = null,
    val expirationDate: Date? = null,
    val additionalProperties: Map<String, Any>? = null
)

/**
 * Represents a Verifiable Presentation according to the [W3C Verifiable Credentials Data Model v1.1](https://www.w3.org/TR/vc-data-model/#presentations-0)
 * and conformant to the [Web5 specification](https://tbd54566975.github.io/web5-spec/#verifiable-presentation-v11-data-model).
 * A Verifiable Presentation allows a holder to present one or more credentials.
 *
 * @property context A list of contexts used to define the semantic meaning of the data contained in the presentation.
 * @property type The type(s) of the Verifiable Presentation.
 * @property id The unique identifier for the Verifiable Presentation.
 * @property holder The entity holding and presenting the Verifiable Presentation, identified by a DID or other identifier.
 * @property issuanceDate The date and time when the presentation was issued.
 * @property expirationDate The optional expiration date and time after which the presentation is no longer valid.
 * @property verifiableCredential A list of Verifiable Credentials contained within the presentation.
 * @property additionalProperties Additional data that may be included in the presentation, represented as a key-value map.
 */
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
        /**
         * Creates a new Verifiable Presentation with the specified holder, Verifiable Credential JWTs,
         * and optional creation options.
         *
         * @param holder The entity holding and presenting the Verifiable Presentation. The holder must be a valid DID.
         * @param verifiableCredential A list of Verifiable Credential JWTs to include in the presentation.
         * @param options Optional parameters for creating the presentation, such as context or expiration.
         * @return The newly created Verifiable Presentation.
         * @throws Web5Exception if the creation fails due to invalid parameters or verification issues.
         *
         * Example usage:
         * ```
         * val holderBearerDid = DidJwk.create()
         * val vcJwts = listOf("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw")
         * val presentation = VerifiablePresentation.create(
         *     holderBearerDid.did.uri,
         *     vcJwts
         * )
         * ```
         */
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

        /**
         * Constructs a Verifiable Presentation from a VP JWT (JSON Web Token).
         *
         * @param vpJwt The Verifiable Presentation in JWT format, serialized as a compact JWS.
         * @param verify If true, verifies the integrity of the JWT by performing cryptographic verification against the signature and validating the Data Model.
         * @return The deserialized and validated Verifiable Presentation.
         * @throws Web5Exception if the JWT is invalid or verification fails.
         *
         * Example usage:
         * ```
         * val vpJwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKYWNUaFJaR05XYlRrMlluZGpRa3R3WVhwV2RGQmlkekJ6U1c4NE0wbG9XRXAyVGtoV1VIUnpWWFYzSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUphY1RoUlpHTldiVGsyWW5kalFrdHdZWHBXZEZCaWR6QnpTVzg0TTBsb1dFcDJUa2hXVUhSelZYVjNJbjAiLCJqdGkiOiJ1cm46dXVpZDowZDg5YTcxMS0zNTdjLTQzNTQtOWYzMS02OWQ0NDE1NWQ1ZTMiLCJuYmYiOjE3MjYyMzQwODEsImlhdCI6MTcyNjIzNDA4MSwidnAiOnsiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMzoyODowMVoiLCJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVQcmVzZW50YXRpb24iXSwidmVyaWZpYWJsZUNyZWRlbnRpYWwiOlsiZXlKMGVYQWlPaUpLVjFRaUxDSmhiR2NpT2lKRlpESTFOVEU1SWl3aWEybGtJam9pWkdsa09tcDNhenBsZVVwb1lrZGphVTlwU2taYVJFa3hUbFJGTlVscGQybGhNMUkxU1dwdmFWUXdkRkZKYVhkcFdUTktNa2xxYjJsU1YxRjVUbFJWZUU5VFNYTkpibWRwVDJsS1VWRnNiRTVTYlRreFdUQnpOVk16WkZCVFNGSjZUbXBvVTA1RlZuZGpiVmw1VFhwT1RFNVVVazFOVmxaSlRqRlNTV05VVW1aaE1HaE9TVzR3SXpBaWZRLmV5SnBjM01pT2lKa2FXUTZhbmRyT21WNVNtaGlSMk5wVDJsS1JscEVTVEZPVkVVMVNXbDNhV0V6VWpWSmFtOXBWREIwVVVscGQybFpNMG95U1dwdmFWSlhVWGxPVkZWNFQxTkpjMGx1WjJsUGFVcFJVV3hzVGxKdE9URlpNSE0xVXpOa1VGTklVbnBPYW1oVFRrVldkMk50V1hsTmVrNU1UbFJTVFUxV1ZrbE9NVkpKWTFSU1ptRXdhRTVKYmpBaUxDSnFkR2tpT2lKMWNtNDZkWFZwWkRwaE1UaGlOREppWVMwMk1UVTVMVFExWVRrdFlXTXpZaTB5TnpaaVlqQmtORGRpWmpZaUxDSnpkV0lpT2lKa2FXUTZaR2gwT201bk5HaHRjWFJ5WjNWcWIzZzBZV2R3WmpodmEzaHBhRzU1ZVRGNmNXNXhPVGR4Wm1WeE1UVjRPRzloY2pkNVpYQjZhSGtpTENKdVltWWlPakUzTWpZeU16RTVOeklzSW1saGRDSTZNVGN5TmpJek1UazNNaXdpZG1NaU9uc2lRR052Ym5SbGVIUWlPbHNpYUhSMGNITTZMeTkzZDNjdWR6TXViM0puTHpJd01UZ3ZZM0psWkdWdWRHbGhiSE12ZGpFaVhTd2lZM0psWkdWdWRHbGhiRk4xWW1wbFkzUWlPbnNpYVdRaU9pSmthV1E2WkdoME9tNW5OR2h0Y1hSeVozVnFiM2cwWVdkd1pqaHZhM2hwYUc1NWVURjZjVzV4T1RkeFptVnhNVFY0T0c5aGNqZDVaWEI2YUhraWZTd2lhWE56ZFdWeUlqb2laR2xrT21wM2F6cGxlVXBvWWtkamFVOXBTa1phUkVreFRsUkZOVWxwZDJsaE0xSTFTV3B2YVZRd2RGRkphWGRwV1ROS01rbHFiMmxTVjFGNVRsUlZlRTlUU1hOSmJtZHBUMmxLVVZGc2JFNVNiVGt4V1RCek5WTXpaRkJUU0ZKNlRtcG9VMDVGVm5kamJWbDVUWHBPVEU1VVVrMU5WbFpKVGpGU1NXTlVVbVpoTUdoT1NXNHdJaXdpYVhOemRXRnVZMlZFWVhSbElqb2lNakF5TkMwd09TMHhNMVF4TWpvMU1qbzFNbG9pTENKMGVYQmxJanBiSWxabGNtbG1hV0ZpYkdWRGNtVmtaVzUwYVdGc0lsMHNJbWxrSWpvaWRYSnVPblYxYVdRNllURTRZalF5WW1FdE5qRTFPUzAwTldFNUxXRmpNMkl0TWpjMlltSXdaRFEzWW1ZMkluMTkuaUNkN1FsQWlCTkxDZnZ0VWJCdGstOVBUcUZmdWNxWjQ0S3hoRnZqR2NSU2prR0pyNjEwLTBqTFZzTlNBX0NQOGdibFljZncxZTVqeDNwR2VFckMtQnciXSwiaG9sZGVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKYWNUaFJaR05XYlRrMlluZGpRa3R3WVhwV2RGQmlkekJ6U1c4NE0wbG9XRXAyVGtoV1VIUnpWWFYzSW4wIiwiaWQiOiJ1cm46dXVpZDowZDg5YTcxMS0zNTdjLTQzNTQtOWYzMS02OWQ0NDE1NWQ1ZTMifX0.f-kdfbIIms3Gg2dMKUMayeU1rQnaO_o0io33kLzy-uPqI6vsdsJZvSmDIilx7scRqlia7Pmnnj6bnF2x8F2fAw"
         * val presentation = VerifiablePresentation.fromVpJwt(vpJwt, true)
         * ```
         */
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

    /**
     * Signs the Verifiable Presentation using the specified Bearer DID and optional verification method.
     *
     * @param bearerDid The DID used to sign the presentation.
     * @param verificationMethodId Optional identifier of the Verification Method to sign with.
     * @return A string representing the signed JWT, serialized as a compact JWS, of the Verifiable Presentation.
     * @throws Web5Exception if the signing process fails.
     *
     * Example usage:
     * ```
     * val holderBearerDid = DidJwk.create()
     * val vcJwts = listOf("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZDI1NTE5Iiwia2lkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIzAifQ.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpESTFOVEU1SWl3aWEzUjVJam9pVDB0UUlpd2lZM0oySWpvaVJXUXlOVFV4T1NJc0luZ2lPaUpRUWxsTlJtOTFZMHM1UzNkUFNIUnpOamhTTkVWd2NtWXlNek5MTlRSTU1WVklOMVJJY1RSZmEwaE5JbjAiLCJqdGkiOiJ1cm46dXVpZDphMThiNDJiYS02MTU5LTQ1YTktYWMzYi0yNzZiYjBkNDdiZjYiLCJzdWIiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkiLCJuYmYiOjE3MjYyMzE5NzIsImlhdCI6MTcyNjIzMTk3MiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6ZGh0Om5nNGhtcXRyZ3Vqb3g0YWdwZjhva3hpaG55eTF6cW5xOTdxZmVxMTV4OG9hcjd5ZXB6aHkifSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaREkxTlRFNUlpd2lhM1I1SWpvaVQwdFFJaXdpWTNKMklqb2lSV1F5TlRVeE9TSXNJbmdpT2lKUVFsbE5SbTkxWTBzNVMzZFBTSFJ6TmpoU05FVndjbVl5TXpOTE5UUk1NVlZJTjFSSWNUUmZhMGhOSW4wIiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wOS0xM1QxMjo1Mjo1MloiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl0sImlkIjoidXJuOnV1aWQ6YTE4YjQyYmEtNjE1OS00NWE5LWFjM2ItMjc2YmIwZDQ3YmY2In19.iCd7QlAiBNLCfvtUbBtk-9PTqFfucqZ44KxhFvjGcRSjkGJr610-0jLVsNSA_CP8gblYcfw1e5jx3pGeErC-Bw")
     * val presentation = VerifiablePresentation.create(
     *     holderBearerDid.did.uri,
     *     vcJwts
     * )
     * val vpJwt = presentation.sign(holderBearerDid)
     * ```
     */
    fun sign(bearerDid: BearerDid, verificationMethodId: String? = null): String {
        try {
            return rustCoreVerifiablePresentation.sign(bearerDid.rustCoreBearerDid, verificationMethodId)
        } catch (e: RustCoreException) {
            throw Web5Exception.fromRustCore(e)
        }
    }
}
