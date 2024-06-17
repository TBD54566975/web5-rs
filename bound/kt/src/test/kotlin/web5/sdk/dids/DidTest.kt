package web5.sdk

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test
import web5.sdk.dids.Did

class DidTest {

  @Test
  fun `toString() returns url`() {
    val did = Did(
      uri = "did:example:123",
      url = "did:example:123#0",
      method = "example",
      id = "123",
      )
    assertEquals("did:example:123#0", did.url)
  }
//
//  @Test
//  fun `Parser throws exception with invalid did`() {
//    val invalidDids = listOf(
//      "",
//      "did:",
//      "did:uport",
//      "did:uport:",
//      "did:uport:1234_12313***",
//      "2nQtiQG6Cgm1GYTBaaKAgr76uY7iSexUkqX",
//      "did:method:%12%1",
//      "did:method:%1233%Ay",
//      "did:CAP:id",
//      "did:method:id::anotherid%r9")
//    for (did in invalidDids) {
//      val exception = assertThrows {
//        Did.Parser.parse(did)
//      }
//      assertEquals("Invalid DID URI", exception.message)
//    }
//  }
//
//  @Test
//  fun `Parser parses a valid did`() {
//    // TODO adding /path after abcdefghi messes up the parsing of params (comes in null)
//    //  https://github.com/TBD54566975/web5-spec/issues/120
//    val did = Did.Parser.parse("did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1")
//    assertEquals("did:example:123456789abcdefghi", did.uri)
//    assertEquals("123456789abcdefghi", did.id)
//    assertEquals("did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1", did.url)
//    assertEquals("example", did.method)
//    assertEquals("123456789abcdefghi", did.id)
//    assertEquals("foo=bar&baz=qux", did.query)
//    assertEquals("keys-1", did.fragment)
//    assertEquals(mapOf("foo" to "bar", "baz" to "qux"), did.params)
//  }

}