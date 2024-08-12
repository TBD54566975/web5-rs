package web5.sdk.rust

import org.junit.jupiter.api.Test

class SystemTargetTest {
    @Test
    fun `can load shared library`() {
        System.setProperty("WEB5_SDK_LOG_LEVEL", "debug")
        UniffiLib.INSTANCE
        println("Successfully loaded shared library for ${detectSystemTarget()}")
    }
}