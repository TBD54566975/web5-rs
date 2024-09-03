package web5.sdk.rust

import java.io.File
import java.net.URLDecoder

internal val logLevel = System.getenv("WEB5_SDK_LOG_LEVEL")?.lowercase()

internal fun log(message: String) {
    if (logLevel == "debug") {
        println("web5 sdk SystemArchitecture $message")
    }
}

internal fun setJNALibraryPath() {
    // Get the class loader resource URL
    val classLoader = Thread.currentThread().contextClassLoader
    val resource = classLoader.getResource("web5_uniffi_x86_64_pc_windows_msvc.dll")

    if (resource != null) {
        // Decode URL to handle spaces and special characters
        val decodedPath = URLDecoder.decode(resource.path, "UTF-8")
        val file = File(decodedPath).parentFile

        // Set the JNA library path
        System.setProperty("jna.library.path", file.absolutePath)
    } else {
        throw IllegalStateException("Native library not found in resources!")
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

        name.contains("windows") -> {
            setJNALibraryPath()
            return "web5_uniffi_x86_64_pc_windows_msvc"
        }

        else -> throw Exception("Unsupported OS arch $arch $name")
    }
}