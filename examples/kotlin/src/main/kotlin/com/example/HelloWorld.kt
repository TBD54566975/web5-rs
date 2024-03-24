package com.example

import java.io.File

object LibraryLoader {
  init {
      val libraryName = "libjwk.dylib"
      val resourcePath = "/natives/$libraryName"
      val input = LibraryLoader::class.java.getResourceAsStream(resourcePath)
      input?.let {
          val tempFile = File.createTempFile("libprefix-", "-libsuffix.dylib")
          tempFile.deleteOnExit()
          tempFile.outputStream().use { output -> input.copyTo(output) }
          System.load(tempFile.absolutePath)
      } ?: throw UnsatisfiedLinkError("Could not load native library $libraryName")
  }
}

fun main(args: Array<String>) {
    println("Hello, World!")
    LibraryLoader // Ensures the native library is loaded
    val jwk = Jwk()
    val thumbprint = jwk.computeThumbprint()
    println("Computed thumbprint: $thumbprint")
}
