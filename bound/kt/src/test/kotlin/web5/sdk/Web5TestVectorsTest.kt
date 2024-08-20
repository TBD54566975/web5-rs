package web5.sdk

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.PropertyNamingStrategy
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.databind.cfg.MapperConfig
import com.fasterxml.jackson.databind.introspect.AnnotatedField
import com.fasterxml.jackson.databind.introspect.AnnotatedMethod
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import org.junit.jupiter.api.Test
import java.io.File
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Nested
import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.crypto.verifiers.Ed25519Verifier
import web5.sdk.dids.Document
import web5.sdk.rust.DocumentMetadataData
import web5.sdk.rust.ResolutionMetadataData

class Web5TestVectorsProtocolTest {

    class TestVectors<I, O>(
        val description: String,
        val vectors: List<TestVector<I, O>>
    )

    class TestVector<I, O>(
        val description: String,
        val input: I,
        val output: O?,
        val errors: Boolean? = false,
    )

    data class SignTestInput(
        val data: String,
        val key: TestVectorJwk,
    )

    data class TestVectorJwk(
        val crv: String,
        val d: String?,
        val kid: String,
        val kty: String,
        val x: String
    )

    data class VerifyTestInput(
        val key: Map<String, Any>,
        val signature: String,
        val data: String
    )

    data class DidJwkResolveTestOutput(
        val context: String?,
        val didDocument: Document?,
        val didDocumentMetadata: DocumentMetadataData,
        val didResolutionMetadata: ResolutionMetadataData?
    )

    @Nested
    inner class Web5TestVectorsCryptoEd25519 {
        @Test
        fun sign() {
            val typeRef = object : TypeReference<TestVectors<SignTestInput, String>>() {}
            val testVectors =
                Json.jsonMapper.readValue(File("../../web5-spec/test-vectors/crypto_ed25519/sign.json"), typeRef)

            testVectors.vectors.forEach { vector ->
                val inputByteArray = hexStringToByteArray(vector.input.data)
                val testVectorJwk = vector.input.key

                val ed25519Jwk = Jwk(
                    kty = testVectorJwk.kty,
                    crv = testVectorJwk.crv,
                    d = testVectorJwk.d,
                    x = testVectorJwk.x,
                    alg = null,
                    y = null
                )
                val signer = Ed25519Signer(ed25519Jwk)

                if (vector.errors == true) {
                    assertThrows(Exception::class.java) {
                        signer.sign(inputByteArray)
                    }
                } else {
                    val signedByteArray = signer.sign(inputByteArray)
                    val signedHex = byteArrayToHexString(signedByteArray)
                    assertEquals(vector.output, signedHex)
                }
            }
        }

        @Test
        fun verify() {
            val typeRef = object : TypeReference<TestVectors<VerifyTestInput, Boolean>>() {}
            val testVectors =
                Json.jsonMapper.readValue(File("../../web5-spec/test-vectors/crypto_ed25519/verify.json"), typeRef)

            testVectors.vectors.forEach { vector ->
                val inputByteArray = hexStringToByteArray(vector.input.data)
                val signatureByteArray = hexStringToByteArray(vector.input.signature)
                val testVectorJwk = Json.jsonMapper.convertValue(vector.input.key, TestVectorJwk::class.java)

                val ed25519Jwk = Jwk(
                    kty = testVectorJwk.kty,
                    crv = testVectorJwk.crv,
                    d = null,
                    x = testVectorJwk.x,
                    alg = null,
                    y = null
                )
                val verifier = Ed25519Verifier(ed25519Jwk)

                if (vector.errors == true) {
                    assertThrows(Exception::class.java) {
                        verifier.verify(inputByteArray, signatureByteArray)
                    }
                } else {
                    val verified = verifier.verify(inputByteArray, signatureByteArray)
                    assertEquals(vector.output, verified)
                }
            }
        }

        private fun hexStringToByteArray(s: String): ByteArray {
            val len = s.length
            val data = ByteArray(len / 2)
            for (i in 0 until len step 2) {
                data[i / 2] = ((Character.digit(s[i], 16) shl 4) + Character.digit(s[i + 1], 16)).toByte()
            }
            return data
        }

        private fun byteArrayToHexString(bytes: ByteArray): String {
            return bytes.joinToString("") { "%02x".format(it) }
        }
    }

    @Nested
    inner class Web5TestVectorsDidJwk {

        // This is so we can parse the test vector converting metadata errors in the test vectors like invalidDid to the INVALID-DID enum
        internal inner class CustomEnumNamingStrategy : PropertyNamingStrategy() {
            override fun nameForField(config: MapperConfig<*>?, field: AnnotatedField?, defaultName: String?): String? {
                if (field?.type?.isEnumType == true) {
                    return convertToUpperSnakeCase(defaultName)
                }
                return defaultName
            }

            override fun nameForGetterMethod(config: MapperConfig<*>?, method: AnnotatedMethod?, defaultName: String?): String {
                if (method?.rawReturnType?.isEnum == true) {
                    return convertToUpperSnakeCase(defaultName)
                }
                return defaultName!!
            }

            override fun nameForSetterMethod(config: MapperConfig<*>?, method: AnnotatedMethod?, defaultName: String?): String {
                if (method?.rawParameterTypes?.firstOrNull()?.isEnum == true) {
                    return convertToUpperSnakeCase(defaultName)
                }
                return defaultName!!
            }

            private fun convertToUpperSnakeCase(input: String?): String {
                return input?.replace("([a-z])([A-Z]+)".toRegex(), "$1_$2")?.uppercase() ?: ""
            }
        }

        val jsonMapper: ObjectMapper = jacksonObjectMapper()
            .registerKotlinModule()
            .findAndRegisterModules()
            .setPropertyNamingStrategy(CustomEnumNamingStrategy())
            .setSerializationInclusion(JsonInclude.Include.NON_NULL)
            .disable((DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES))
            .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)

        @Test
        fun resolve() {
            val typeRef = object : TypeReference<TestVectors<String, DidJwkResolveTestOutput>>() {}

            val testVectors =
                jsonMapper.readValue(File("../../web5-spec/test-vectors/did_jwk/resolve.json"), typeRef)

            testVectors.vectors.forEach { vector ->
                if (vector.errors == true) {
                    val resolvedDid = web5.sdk.dids.methods.jwk.DidJwk.resolve(vector.input)

                    assertTrue(resolvedDid.resolutionMetadata.error != null)

                    // TODO: parse resolutionMetadata from the test vector correctly
//                    assertEquals(resolvedDid.resolutionMetadata, vector.output!!.didResolutionMetadata)
                } else {
                    val resolvedDid = web5.sdk.dids.methods.jwk.DidJwk.resolve(vector.input)

                    assertEquals(resolvedDid.document!!.id, vector.output!!.didDocument!!.id)
                    assertEquals(resolvedDid.document!!.verificationMethod, vector.output.didDocument!!.verificationMethod)
                    assertEquals(resolvedDid.document!!.authentication, vector.output.didDocument.authentication)
                    assertEquals(resolvedDid.document!!.assertionMethod, vector.output.didDocument.assertionMethod)
                    assertEquals(resolvedDid.document!!.capabilityDelegation, vector.output.didDocument.capabilityDelegation)
                    assertEquals(resolvedDid.document!!.capabilityInvocation, vector.output.didDocument.capabilityInvocation)
                }
            }
        }
    }
}