// The Swift Programming Language
// https://docs.swift.org/swift-book

import jwkFFI // Import the system library target wrapping your Rust library.

print("Hello, world!")

// Use the Rust library functions or types.
let jwkInstance = Jwk() // Assuming `JWK` is a class or struct provided by your Rust library.
let thumbprint = jwkInstance.computeThumbprint() // Call a method provided by your Rust library.

print(thumbprint) // Print the result to verify everything is working.
