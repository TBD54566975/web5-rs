package com.example

import web5.sdk.*
import java.util.Base64

fun main(args: Array<String>) {
     val jwk = Jwk(
         "ES256K",
         "EC",
         "secp256k1",
         "P3hRuve79GaggsVdQG_w-JpdM6dHXG33-1nwZ8Jw07g",
         "vA8umEbOhhQjFfk1-byvVxtJNRtwQSEE0UMVmxSN9K4",
         "A1qGUBx-wpznzVI0DLu8kEhDZ77ou533NKSCw90R33Q")
     println(jwk)

    val thumbprint = computeThumbprint(jwk)
    println("Computed thumbprint: $thumbprint")

    val ed25199Jwk = ed25519Generate()
    val ed25199Thumbprint = computeThumbprint(ed25199Jwk)
    println("Computed thumbprint (Ed25519): $ed25199Thumbprint")

    val payload = "hello world".toByteArray()
    val signature = ed25519Sign(ed25199Jwk, payload)
    println("Signature ${Base64.getEncoder().encodeToString(signature)}")

    ed25519Verify(ed25199Jwk, payload, signature)
    println("verify() passed as expected")
    try {
        ed25519Verify(ed25199Jwk, payload, "invalid sig".toByteArray())
    } catch (e: Exception) {
        println("verify() failed as expected")
    }

    val keyManager = LocalJwkManager()
    val keyAlias = keyManager.generatePrivateKey(Curve.ED25519, null)
    println("Key Alias: $keyAlias")
    val publicKey = keyManager.getPublicKey(keyAlias)
    println("Public Key: $publicKey")
    val signature2 = keyManager.sign(keyAlias, payload)
    ed25519Verify(publicKey, payload, signature2)
    println("Signed & verified ${Base64.getEncoder().encodeToString(signature2)}")
    val privateKeys = keyManager.exportPrivateKeys()
    println("Exported private keys $privateKeys")
    keyManager.importPrivateKeys(privateKeys)
    println("Imported private keys")

    val identifier = identifierParse("did:example:123456789abcdefghi;foo=bar;baz=qux?foo=bar&baz=qux#keys-1")
    println(identifier)
}