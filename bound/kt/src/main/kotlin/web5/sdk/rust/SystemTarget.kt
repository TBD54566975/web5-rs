package web5.sdk.rust

import java.io.File

object SystemTarget {
    @Volatile
    private var isSet = false
    private var gitCommitHash = ""

    fun set() {
        val logLevel = System.getenv("WEB5_SDK_LOG_LEVEL")?.lowercase()

        val commitFile = File("target/git-commit-id.txt")
        if (commitFile.exists()) {
            gitCommitHash = commitFile.readText().trim()
        } else {
            println("Git commit hash not found.")
        }

        fun log(message: String) {
            if (logLevel == "debug") {
                println("web5 sdk SystemArchitecture $gitCommitHash: $message")
            }
        }

        if (!isSet) {
            synchronized(this) {
                if (!isSet) {
                    val arch = System.getProperty("os.arch")?.lowercase() ?: throw Exception("Unable to get OS arch")
                    val name = System.getProperty("os.name")?.lowercase() ?: throw Exception("Unable to get OS name")

                    log("System architecture: $arch")
                    log("Operating system name: $name")

                    when {
                        name.contains("mac") && arch.contains("aarch64") ->
                            System.setProperty("uniffi.component.web5.libraryOverride", "web5_uniffi_aarch64_apple_darwin")

                        name.contains("mac") && arch.contains("x86_64") ->
                            System.setProperty("uniffi.component.web5.libraryOverride", "web5_uniffi_x86_64_apple_darwin")

                        name.contains("linux") && arch.contains("amd64") -> {
                            val osRelease = File("/etc/os-release")
                            if (osRelease.exists()) {
                                val osReleaseContent = osRelease.readText().lowercase()
                                log("OS release content: $osReleaseContent")
                                when {
                                    osReleaseContent.contains("ubuntu") ->
                                        System.setProperty("uniffi.component.web5.libraryOverride", "web5_uniffi_x86_64_unknown_linux_gnu")

                                    osReleaseContent.contains("alpine") ->
                                        System.setProperty("uniffi.component.web5.libraryOverride", "web5_uniffi_x86_64_unknown_linux_musl")

                                    else -> throw Exception("Unsupported OS arch $osReleaseContent")
                                }
                            } else {
                                throw Exception("Linux /etc/os-release not found")
                            }
                        }

                        else -> throw Exception("Unsupported OS arch $arch $name")
                    }
                    isSet = true
                }
            }
        }
    }
}