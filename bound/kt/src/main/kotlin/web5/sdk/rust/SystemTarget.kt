package web5.sdk.rust

import java.io.File

internal val logLevel = System.getenv("WEB5_SDK_LOG_LEVEL")?.lowercase()

internal fun log(message: String) {
    if (logLevel == "debug") {
        println("web5 sdk SystemArchitecture $message")
    }
}

internal fun detectSystemTarget(): String {
    val arch = System.getProperty("os.arch")?.lowercase() ?: throw Exception("Unable to get OS arch")
    val name = System.getProperty("os.name")?.lowercase() ?: throw Exception("Unable to get OS name")

    log("System architecture: $arch")
    log("Operating system name: $name")

    when {
        name.contains("mac") && arch.contains("aarch64") ->
            return "web5_uniffi_aarch64_apple_darwin"

        name.contains("mac") && arch.contains("x86_64") ->
            return "web5_uniffi_x86_64_apple_darwin"

        name.contains("linux") && arch.contains("amd64") -> {
            val osRelease = File("/etc/os-release")
            if (osRelease.exists()) {
                val osReleaseContent = osRelease.readText().lowercase()
                log("OS release content: $osReleaseContent")
                return when {
                    osReleaseContent.contains("ubuntu") ->
                        "web5_uniffi_x86_64_unknown_linux_gnu"

                    osReleaseContent.contains("alpine") ->
                        "web5_uniffi_x86_64_unknown_linux_musl"

                    else -> throw Exception("Unsupported OS arch $osReleaseContent")
                }
            } else {
                throw Exception("Linux /etc/os-release not found")
            }
        }

        else -> throw Exception("Unsupported OS arch $arch $name")
    }
}