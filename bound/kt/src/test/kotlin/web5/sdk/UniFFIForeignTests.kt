package web5.sdk

import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test

import web5.sdk.rust.ExampleForeignTrait
import web5.sdk.rust.exampleForeignTrait

class Example : ExampleForeignTrait {
    override fun helloWorld() {
        println("hello from kt but inside of rust")
    }

    override fun helloWorld2() {
        println("hello v2 from kt but inside of rust")
    }

    override fun helloWorld3() {
        println("hello v3 from kt but inside of rust")
    }
}

class UniFFIForeignTests {
    @Test
    fun `test foreign implementation`() {
        val ex = Example()
        exampleForeignTrait(ex)
    }
}