package web5.sdk.dids

import org.junit.jupiter.api.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.fail
import web5.sdk.UnitTestSuite
import web5.sdk.Web5Exception

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class DidTest {

  @Test
  fun test_did_empty_string_should_error() {
    val uri = ""

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_incomplete_scheme_should_error() {
    testSuite.include()
    val uri = "did:"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_missing_id_part_should_error() {
    testSuite.include()
    val uri = "did:uport"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_missing_id_should_error() {
    testSuite.include()
    val uri = "did:uport:"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_invalid_characters_in_id_should_error() {
    testSuite.include()
    val uri = "did:uport:1234_12313***"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_invalid_bare_id_should_error() {
    testSuite.include()
    val uri = "2nQtiQG6Cgm1GYTBaaKAgr76uY7iSexUkqX"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_invalid_percent_encoding_should_error() {
    testSuite.include()
    val uri = "did:method:%12%1"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_invalid_percent_encoding_incomplete_should_error() {
    testSuite.include()
    val uri = "did:method:%1233%Ay"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_capitalized_method_should_error() {
    testSuite.include()
    val uri = "did:CAP:id"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_invalid_additional_id_should_error() {
    testSuite.include()
    val uri = "did:method:id::anotherid%r9"

    val exception = assertThrows<Web5Exception> {
      Did.parse(uri)
    }

    assertEquals("parameter error identifier regex match failure $uri", exception.message)
  }

  @Test
  fun test_did_valid_did_no_params_path_query_fragment() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi"
    val expected = Did(
      uri = uri,
      url = uri,
      method = "example",
      id = "123456789abcdefghi"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_params() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi;foo=bar;baz=qux"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      params = mapOf("foo" to "bar", "baz" to "qux")
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_query() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi?foo=bar&baz=qux"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      query = "foo=bar&baz=qux"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_fragment() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi#keys-1"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      fragment = "keys-1"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_query_and_fragment() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi?foo=bar&baz=qux#keys-1"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      query = "foo=bar&baz=qux",
      fragment = "keys-1"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_params_query_and_fragment() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      params = mapOf("foo" to "bar", "baz" to "qux"),
      query = "foo=bar&baz=qux",
      fragment = "keys-1"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }

  @Test
  fun test_did_valid_did_with_path() {
    testSuite.include()
    val uri = "did:example:123456789abcdefghi/path/to/resource"
    val expected = Did(
      uri = "did:example:123456789abcdefghi",
      url = uri,
      method = "example",
      id = "123456789abcdefghi",
      path = "/path/to/resource"
    )
    val result = Did.parse(uri)
    assertEquals(expected, result)
  }
}
