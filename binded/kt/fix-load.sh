#!/bin/bash

# Path to your Kotlin file
FILE="src/main/kotlin/web5/sdk/web5.kt"

# Add the import line after the last import
awk '/^import/ {print; found=1; next} found && !/^import/ {print "import java.nio.file.Files"; found=0} {print}' $FILE > tmpfile && mv tmpfile $FILE

# Replace the specified block of code using a more BSD-friendly approach
sed -i '' '/loadIndirect<UniffiLib>(componentName = "web5")/,/also { lib: UniffiLib ->/c\
            val osName = System.getProperty("os.name").lowercase()\
            val libFileName = when {\
                osName.contains("mac") -> "libweb5_uniffi.dylib"\
                osName.contains("nux") || osName.contains("nix") -> "libweb5_uniffi.so"\
                else -> throw UnsupportedOperationException("Unsupported operating system: $osName")\
            }\
            val tempDir = Files.createTempDirectory("library")\
            val libraryPath = tempDir.resolve(libFileName)\
            Thread.currentThread().contextClassLoader.getResourceAsStream("natives/$libFileName").use { input ->\
                Files.copy(input, libraryPath)\
            }\
            libraryPath.toFile().deleteOnExit()\
            val lib = Native.load(libraryPath.toString(), UniffiLib::class.java)\
            lib.also {' $FILE
