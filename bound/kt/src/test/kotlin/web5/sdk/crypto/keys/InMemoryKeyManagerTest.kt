package web5.sdk.crypto.keys

import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertNotNull

import org.junit.jupiter.api.Test
import web5.sdk.rust.RustCoreException
import web5.sdk.rust.ed25519GeneratorGenerate

class InMemoryKeyManagerTest {

  @Test
  fun `test key manager`() {
    val keyManager = InMemoryKeyManager()

    val jwk = keyManager.generateKeyMaterial()
    val signer = keyManager.getSigner(jwk)

    val payload = signer.sign("abc".toByteArray())

    assertNotNull(payload)
  }

  @Test
  fun `test wrong jwk for key manager`() {
    val jwk = Jwk(alg="Ed25519", kty="OKP", crv="Ed25519", d=null, x="yxTpaqbGhLNMfOCu31znPNNei0OtDiQ_AS9DxC7Bstg", y=null)
    val keyManager = InMemoryKeyManager()

    assertThrows(RustCoreException::class.java) {
      keyManager.getSigner(jwk)
    }
  }

  @Test
  fun `test key manager import key`() {
    val rustCorePrivateJwk = ed25519GeneratorGenerate()

    val keyManager = InMemoryKeyManager()
    keyManager.importKey(rustCorePrivateJwk)

    rustCorePrivateJwk.d = null
    val signer = keyManager.getSigner(rustCorePrivateJwk)

    val payload = signer.sign("abc".toByteArray())

    assertNotNull(payload)
  }
}