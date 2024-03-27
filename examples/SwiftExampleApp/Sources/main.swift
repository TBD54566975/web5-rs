import UniFFI

// TODO rather than this syntax, we could wrap the UniFFI/jwk.swift code into something more idiomatic in Web5/web5.swift

let jwk = UniFFI.Jwk(alg: "", kty: "EC", crv: "secp256k1", d: "", x: "IP76NWyz81Bk1Zfsbk_ZgTJ57nTMIGM_YKdUlAUKbeY", y: "UefbWznggYPo3S17R9hcW5wAmwYoyfFw9xeBbQOacaA")
let thumbprint = try computeThumbprint(jwk: jwk)
print("Thumbprint: \(thumbprint)")
