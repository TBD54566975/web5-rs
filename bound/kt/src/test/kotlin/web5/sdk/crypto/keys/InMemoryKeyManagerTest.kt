package web5.sdk.crypto.keys

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test

class InMemoryKeyManagerTest {

  @Test
  fun `test key manager`() {
    val keyManager = InMemoryKeyManager()

    val jwk = keyManager.generateKeyMaterial()
    val signer = keyManager.getSigner(jwk)

    assertNotNull(jwk)
    assertNotNull(signer)
  }
}