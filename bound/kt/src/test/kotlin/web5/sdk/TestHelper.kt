package web5.sdk

import java.nio.file.Files
import java.nio.file.Paths

class UnitTestSuite(name: String) {
    val tests: MutableList<String>

    init {
        val path = Paths.get("../../tests/unit_test_cases/$name.json")
        val jsonString = Files.readString(path)
        this.tests = Json.jsonMapper.readValue(jsonString, List::class.java) as MutableList<String>
    }

    fun include() {
        val testMethodName = Thread.currentThread().stackTrace
            .firstOrNull { it.methodName.startsWith("test") }?.methodName
            ?: throw IllegalStateException("Unable to determine test method name")

        this.tests.remove(testMethodName)
    }
}