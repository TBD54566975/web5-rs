package web5.sdk.crypto

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class Ed25519GeneratorTest {

    @Test
    fun test_must_set_alg() {

        val jwk = Ed25519Generator.generate()
        assertEquals("Ed25519", jwk.alg)
    }

    @Test
    fun test_must_set_kty() {

        val jwk = Ed25519Generator.generate()
        assertEquals("OKP", jwk.kty)
    }

    @Test
    fun test_must_set_crv() {

        val jwk = Ed25519Generator.generate()
        assertEquals("Ed25519", jwk.crv)
    }

    @Test
    fun test_must_set_public_key_with_correct_length() {

        val jwk = Ed25519Generator.generate()
        val publicKeyBytes = java.util.Base64.getUrlDecoder().decode(jwk.x)
        assertEquals(PUBLIC_KEY_LENGTH, publicKeyBytes.size)
    }

    @Test
    fun test_must_set_private_key_with_correct_length() {

        val jwk = Ed25519Generator.generate()
        val privateKeyBytes = jwk.d ?: fail("Private key is missing")
        val decodedPrivateKeyBytes = java.util.Base64.getUrlDecoder().decode(privateKeyBytes)
        assertEquals(SECRET_KEY_LENGTH, decodedPrivateKeyBytes.size)
    }

    companion object {
        const val PUBLIC_KEY_LENGTH = 32
        const val SECRET_KEY_LENGTH = 32
    }
}
