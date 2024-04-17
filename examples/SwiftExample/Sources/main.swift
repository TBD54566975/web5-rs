import UniFFI

let jwk = UniFFI.Jwk(alg: "ES256K", kty: "EC", crv: "secp256k1", d: "P3hRuve79GaggsVdQG_w-JpdM6dHXG33-1nwZ8Jw07g", x: "vA8umEbOhhQjFfk1-byvVxtJNRtwQSEE0UMVmxSN9K4", y: "A1qGUBx-wpznzVI0DLu8kEhDZ77ou533NKSCw90R33Q")

let thumbprint = try jwk.computeThumbprint()
print("Thumbprint: \(thumbprint)")

let ed25519 = UniFFI.Ed25199()
let ed25519Jwk = try ed25519.generate()
let ed25519Thumbprint = try ed25519Jwk.computeThumbprint()
print("Thumbprint (Ed25519): \(ed25519Thumbprint)")
