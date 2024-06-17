package web5.sdk.dids

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DidTest {

  @Test
  fun `test basic did creation`() {
    val did = Did(
      uri = "did:example:123",
      url = "did:example:123#0",
      method = "example",
      id = "123",
      )
    assertEquals("did:example:123#0", did.url)
  }
}