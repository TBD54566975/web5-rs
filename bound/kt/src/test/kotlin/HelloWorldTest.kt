package web5.sdk

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import web5.sdk.helloWorld
import web5.sdk.DidDht
import web5.sdk.InMemoryKeyManager

class LocalKeyManagerTests {
    @Test
    fun `can helloWorld`() {
        helloWorld()
    }
}