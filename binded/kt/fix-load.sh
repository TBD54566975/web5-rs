#!/bin/bash

# Path to your Kotlin file
FILE="src/main/kotlin/web5/sdk/core/web5.kt"

# Add the import line after the last import
awk '/^import/ {print; found=1; next} found && !/^import/ {print "import java.nio.file.Files"; found=0} {print}' $FILE > tmpfile && mv tmpfile $FILE

# Replace the specified block of code using a more BSD-friendly approach
sed -i '' '/loadIndirect<UniffiLib>(componentName = "web5")/,/also { lib: UniffiLib ->/c\
            val tempDir = Files.createTempDirectory("library")\
            val libraryPath = tempDir.resolve("libweb5.dylib")\
            Thread.currentThread().contextClassLoader.getResourceAsStream("natives/libweb5.dylib").use { input ->\
                Files.copy(input, libraryPath)\
            }\
            libraryPath.toFile().deleteOnExit()\
            val lib = Native.load(libraryPath.toString(), UniffiLib::class.java)\
            lib.also {' $FILE
