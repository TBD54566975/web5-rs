package web5.sdk

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.assertEquals
import web5.sdk.ed25519Generate

class Web5Test {
    @Test
    fun testHelloWorld() {
        val result = "Hello, World!"
        assertEquals("Hello, World!", result)
    }

    @Test
    fun testweb5() {
        val result = ed25519Generate()
        println(result)
    }
}