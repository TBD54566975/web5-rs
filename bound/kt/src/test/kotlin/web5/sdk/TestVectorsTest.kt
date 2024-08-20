package web5.sdk

import com.fasterxml.jackson.core.type.TypeReference
import org.junit.jupiter.api.Test
import java.io.File
import org.junit.jupiter.api.Assertions.*
import web5.sdk.crypto.keys.Jwk
import web5.sdk.crypto.signers.Ed25519Signer
import web5.sdk.crypto.verifiers.Ed25519Verifier

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

class Web5TestVectorsCryptoEd25519 {

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

    @Test
    fun sign() {
        val typeRef = object : TypeReference<TestVectors<SignTestInput, String>>() {}
        val testVectors = Json.jsonMapper.readValue(File("../../web5-spec/test-vectors/crypto_ed25519/sign.json"), typeRef)

        testVectors.vectors.forEach { vector ->
            val inputByteArray = hexStringToByteArray(vector.input.data)
            val testVectorJwk = vector.input.key

            val ed25519Jwk = Jwk(kty = testVectorJwk.kty, crv = testVectorJwk.crv, d = testVectorJwk.d, x = testVectorJwk.x, alg = null, y = null)
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

    data class VerifyTestInput(
        val key: Map<String, Any>,
        val signature: String,
        val data: String
    )

    @Test
    fun verify() {
        val typeRef = object : TypeReference<TestVectors<VerifyTestInput, Boolean>>() {}
        val testVectors = Json.jsonMapper.readValue(File("../../web5-spec/test-vectors/crypto_ed25519/verify.json"), typeRef)

        testVectors.vectors.forEach { vector ->
            val inputByteArray = hexStringToByteArray(vector.input.data)
            val signatureByteArray = hexStringToByteArray(vector.input.signature)
            val testVectorJwk = Json.jsonMapper.convertValue(vector.input.key, TestVectorJwk::class.java)

            val ed25519Jwk = Jwk(kty = testVectorJwk.kty, crv = testVectorJwk.crv, d = null, x = testVectorJwk.x, alg = null, y = null)
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
