package web5.sdk.crypto.keys

import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertNotNull

import org.junit.jupiter.api.Test
import web5.sdk.rust.RustCoreException

import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate

class InMemoryKeyManagerTest {

  @Test
  fun `test key manager`() {
    val privateJwk = rustCoreEd25519GeneratorGenerate()

    val keyManager = InMemoryKeyManager()
    val publicJwk = keyManager.importPrivateKey(privateJwk)

    val signer = keyManager.getSigner(publicJwk)
    val payload = signer.sign("abc".map { it.code.toUByte() })

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
    val rustCorePrivateJwk = rustCoreEd25519GeneratorGenerate()

    val keyManager = InMemoryKeyManager()
    keyManager.importPrivateKey(rustCorePrivateJwk)

    rustCorePrivateJwk.d = null
    val signer = keyManager.getSigner(rustCorePrivateJwk)

    val payload = signer.sign("abc".map { it.code.toUByte() })

    assertNotNull(payload)
  }
}