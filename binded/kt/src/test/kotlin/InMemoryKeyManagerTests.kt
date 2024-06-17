package web5.sdk

import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.crypto.keys.InMemoryKeyManager

class InMemoryKeyManagerTests {
    @Test
    fun `can generate key material`() {
        val keyManager = InMemoryKeyManager()

        val jwk = keyManager.generateKeyMaterial()

        assertNotNull(jwk)
    }
}