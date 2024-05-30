# Digital Signature Algorithm <!-- omit in toc -->

- [`KeyManager`](#keymanager)
- [`Dsa` (Enumeration)](#dsa-enumeration)
- [`DsaSigner` (Polymorphic Base Class)](#dsasigner-polymorphic-base-class)
- [`DsaVerifier` (Polymorphic Base Class)](#dsaverifier-polymorphic-base-class)
- [`JwsSigner` (Polymorphic Base Class)](#jwssigner-polymorphic-base-class)
- [`JwsVerifier` (Polymorphic Base Class)](#jwsverifier-polymorphic-base-class)
- [`Ed25519Generator`](#ed25519generator)
- [`Ed25519Signer`](#ed25519signer)
- [`Ed25519Verifier`](#ed25519verifier)
- [Public \& Private Key Material](#public--private-key-material)

# `KeyManager`

Strictly uses Ed25519.

| Instance Method                                 | Notes                                  |
| :---------------------------------------------- | :------------------------------------- |
| `generate_key_material(): Jwk`                  | Return [`Jwk`](#jwk) is a public key.  |
| `get_signer(public_key: &Jwk) -> Ed25519Signer` | See [`Ed25519Signer`](#ed25519signer). |
| `import_keys(private_keys: &Vec<Jwk>)`          |                                        |

# `Dsa` (Enumeration)

The set of Digital Signature Algorithm's natively supported within this SDK.

| Value     |
| :-------- |
| `Ed22519` |

> [!NOTE]
> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

# `DsaSigner` (Polymorphic Base Class)

Private key material must be encapsulated.

| Instance Method                   | Notes |
| :-------------------------------- | :---- |
| `sign(payload: &[u8]) -> Vec<u8>` |       |

# `DsaVerifier` (Polymorphic Base Class)

Public key material must be encapsulated.

| Instance Method                                       | Notes |
| :---------------------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8]) -> Vec<u8>` |       |

# `JwsSigner` (Polymorphic Base Class)

Private key material must be encapsulated.

| Instance Method                   | Notes |
| :-------------------------------- | :---- |
| `sign(payload: &[u8]) -> Vec<u8>` |       |

# `JwsVerifier` (Polymorphic Base Class)

Public key material must be encapsulated.

| Instance Method                                       | Notes |
| :---------------------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8]) -> Vec<u8>` |       |

# `Ed25519Generator`

| Static Method                          | Notes |
| :------------------------------------- | :---- |
| `generate_private_key_material(): Jwk` |       |

# `Ed25519Signer`

Implements [`DsaSigner`](#dsasigner-polymorphic-base-class) and [`JwsSigner`](#jwssigner-polymorphic-base-class) for Ed25519.

| Property           | Notes |
| :----------------- | :---- |
| `private_key: Jwk` |       |

| Constructor                      | Notes |
| :------------------------------- | :---- |
| `constructor(private_key: &Jwk)` |       |

# `Ed25519Verifier`

Implements [`DsaVerifier`](#dsaverifier-polymorphic-base-class) and [`JwsVerifier`](#jwsverifier-polymorphic-base-class) for Ed25519.

| Constructor                     | Notes |
| :------------------------------ | :---- |
| `constructor(public_key: &Jwk)` |       |

# Public & Private Key Material

> [!NOTE]
> We strictly represent public & private *key material* as [JSON Web Key's](./jwk.md), but we may consider disintermediating at some point by introducing polymorphic base classes for `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()` instance method for JWK representations.