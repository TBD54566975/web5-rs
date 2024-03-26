// The Swift Programming Language
// https://docs.swift.org/swift-book

import UniFFI

print("Hello, world!")

let jwk = UniFFI.Jwk()
let thumbprint = jwk.computeThumbprint()
print("Thumbprint: \(thumbprint)")
