package com.example

import java.util.Base64
import web5.sdk.Jwk
import web5.sdk.Ed25199

fun main(args: Array<String>) {
    val jwk = Jwk(
        "ES256K",
        "EC",
        "secp256k1",
        "P3hRuve79GaggsVdQG_w-JpdM6dHXG33-1nwZ8Jw07g",
        "vA8umEbOhhQjFfk1-byvVxtJNRtwQSEE0UMVmxSN9K4",
        "A1qGUBx-wpznzVI0DLu8kEhDZ77ou533NKSCw90R33Q")
    val thumbprint = jwk.computeThumbprint()
    println("Computed thumbprint: $thumbprint")

    val ed25199 = Ed25199()
    val ed25199Jwk = ed25199.generate()
    val ed25199Thumbprint = ed25199Jwk.computeThumbprint()
    println("Computed thumbprint (Ed25519): $ed25199Thumbprint")

    val payload = "hello world".toByteArray()
    val signature = ed25199.sign(ed25199Jwk, payload)
    println("Signature ${Base64.getEncoder().encodeToString(signature)}")

    ed25199.verify(ed25199Jwk, payload, signature)
    println("verify() passed as expected")
    try {
        ed25199.verify(ed25199Jwk, payload, "invalid sig".toByteArray())
    } catch (e: Exception) {
        println("verify() failed as expected")
    }
}