package web5.sdk.crypto.keys

import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertNotNull

import org.junit.jupiter.api.Test
import web5.sdk.rust.Web5Exception

import web5.sdk.rust.ed25519GeneratorGenerate as rustCoreEd25519GeneratorGenerate

class InMemoryKeyManagerTest {

  @Test
  fun `test key manager`() {
    val privateJwk = rustCoreEd25519GeneratorGenerate()

    val keyManager = InMemoryKeyManager(listOf(Jwk.fromRustCoreJwkData(privateJwk)))

    val signer = keyManager.getSigner(Jwk.fromRustCoreJwkData(privateJwk))
    val payload = signer.sign("abc".toByteArray())

    assertNotNull(payload)
  }

  @Test
  fun `test wrong jwk for key manager`() {
    val publicJwk = Jwk(alg="Ed25519", kty="OKP", crv="Ed25519", d=null, x="yxTpaqbGhLNMfOCu31znPNNei0OtDiQ_AS9DxC7Bstg", y=null)

    assertThrows(Web5Exception::class.java) {
      InMemoryKeyManager(listOf(publicJwk))
    }
  }

  @Test
  fun `test key manager import key`() {
    val privateJwk = rustCoreEd25519GeneratorGenerate()

    val keyManager = InMemoryKeyManager(listOf())
    keyManager.importPrivateJwk(Jwk.fromRustCoreJwkData(privateJwk))

    privateJwk.d = null
    val signer = keyManager.getSigner(Jwk.fromRustCoreJwkData(privateJwk))
    val payload = signer.sign("abc".toByteArray())

    assertNotNull(payload)
  }
}