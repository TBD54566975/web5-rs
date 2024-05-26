> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [API Reference](#api-reference)
  - [Examples](#examples)
  - [Cryptography](#cryptography)
    - [`Jwk`](#jwk)
    - [`DsaOps` (Interface)](#dsaops-interface)
    - [`JwsDsaOps` (Interface)](#jwsdsaops-interface)
    - [`Dsa` (Enumeration)](#dsa-enumeration)
    - [`Ed25519`](#ed25519)
    - [`Xd25519`](#xd25519)
    - [`Secp256k1`](#secp256k1)
    - [`Secp256r1`](#secp256r1)
  - [Key Managers](#key-managers)
    - [`InMemoryKeyManager`](#inmemorykeymanager)
    - [`BearerDidKeyManager`](#bearerdidkeymanager)
  - [DIDs](#dids)
    - [`Did`](#did)
    - [`BearerDid`](#bearerdid)
  - [Credentials](#credentials)
    - [`VerifiableCredential`](#verifiablecredential)
  - [Presentation Exchange](#presentation-exchange)
    - [`PresentationDefinition`](#presentationdefinition)

# API Reference

## Examples

🚧 This is under construction, incomplete 🚧

```rust
let key_manager = InMemoryKeyManager::new();
let public_jwk = key_manager.generate_private_key();
let dsa = key_manager.get_dsa(public_jwk);

// create a did
let did_dht = DidDht::create(dsa, public_jwk);

// create a vc, sign it, and verify it
let vc = VerifiableCredential{
  issuer: did_dht.did.uri,
  // todo other things
};
let vcjwt = vc.sign(dsa);
VerifiableCredential::verify(vcjwt, dsa);
```

## Cryptography

### `Jwk`

🚧 Consider constraining in [`web5-spec`](https://github.com/TBD54566975/web5-spec) 🚧

| Property            | Notes                                                                                                                                                |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `alg: String`       |                                                                                                                                                      |
| `kty: String`       |                                                                                                                                                      |
| `crv: String`       |                                                                                                                                                      |
| `d: Option<String>` | 🚧 `d` is private key material, consider removing here but how it works into [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) requirements 🚧 |
| `x: String`         |                                                                                                                                                      |
| `y: Option<String>` |                                                                                                                                                      |

| Instance Method                  | Notes |
| -------------------------------- | ----- |
| `compute_thumbprint() -> String` |       |

### `DsaOps` (Interface)

| Static Method                                                           | Notes                        |
| ----------------------------------------------------------------------- | ---------------------------- |
| `generate_key(): Jwk`                                                   | 🚧 May need to be a keypair 🚧 |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk).           |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk).           |

### `JwsDsaOps` (Interface)

| Static Method                                                           | Notes              |
| ----------------------------------------------------------------------- | ------------------ |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk). |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk). |

### `Dsa` (Enumeration)

| Value       |
| ----------- |
| `Ed22519`   |
| `Xd22519`   |
| `Secp256k1` |
| `Secp256r1` |

### `Ed25519`

Implements [`DsaOps`](#dsaops-interface) and [`JwsDsaOps`](#jwsdsaops-interface) for Ed25519.

### `Xd25519`

Implements [`DsaOps`](#dsaops-interface) and [`JwsDsaOps`](#jwsdsaops-interface) for Xd25519.

### `Secp256k1`

Implements [`DsaOps`](#dsaops-interface) and [`JwsDsaOps`](#jwsdsaops-interface) for secp256k1.

### `Secp256r1`

Implements [`DsaOps`](#dsaops-interface) and [`JwsDsaOps`](#jwsdsaops-interface) for secp256r1.

## Key Managers

### `InMemoryKeyManager` 

Strictly uses Ed25519.

| Instance Method                        | Notes                                                                           |
| :------------------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk`          | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |
| `get_dsa(public_jwk: &Jwk) -> Ed25519` |                                                                                 |

### `BearerDidKeyManager`

| Instance Method                                  | Notes                                                    |
| ------------------------------------------------ | -------------------------------------------------------- |
| `generate_signing_key(alg: Alg) -> String`       | Return string is equal to the key ID.                    |
| `get_public_key(key_id: String) -> Jwk`          | See [`Jwk`](#jwk).                                       |
| `sign(key_id: String, payload: &[u8]) -> &[u8]`  |                                                          |
| `import_private_key(jwk: Jwk) -> String`         | See [`Jwk`](#jwk). Return string is equal to the key ID. |
| `export_private_key(key_id: String) -> Vec<Jwk>` | See [`Jwk`](#jwk).                                       |

## DIDs

Data models conformant to _W3C Decentralized Identifiers v1.0_ [within the `web5-spec`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md).

### `Did`

| Property                                  | Notes |
| :---------------------------------------- | :---- |
| `uri: String`                             |       |
| `url: String`                             |       |
| `method: String`                          |       |
| `id: String`                              |       |
| `params: Option<HashMap<String, String>>` |       |
| `path: Option<String>`                    |       |
| `query: Option<String>`                   |       |
| `fragment: Option<String>`                |       |

| Static Method             | Notes |
| :------------------------ | :---- |
| `parse(uri: &str) -> Did` |       |

### `BearerDid`

| Property                       | Notes |
| ------------------------------ | ----- |
| `did: Did`                     |       |
| `document: Document`           |       |
| `key_manager: LocalKeyManager` |       |

| Static Method                         | Notes |
| ------------------------------------- | ----- |
| `from_serialized(serialized: String)` |       |

| Instance Method                                   | Notes |
| ------------------------------------------------- | ----- |
| `to_serialized() -> Strin`                        |       |
| `get_signer(vm_selector: VmSelector) -> function` | 🚧 🚧 🚧 |

## Credentials

### `VerifiableCredential`

Data models conformant to *W3C Verifiable Credentials v1.1* [within the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                                                          | Notes                              |
| :----------------------------------------------------------------------- | :--------------------------------- |
| `sign(jws_dsa: &dyn JwsDsa) -> String`                                   | See [`JwsDsa`](#jwsdsa-interface). |
| `sign_with_did(bearer_did: BearerDid, options: VcSignOptions) -> String` | 🚧 opts 🚧                           |

| Static Method                                                     | Notes                              |
| :---------------------------------------------------------------- | :--------------------------------- |
| `verify(jwt: &str, jws_dsa: &dyn JwsDsa) -> VerifiableCredential` | See [`JwsDsa`](#jwsdsa-interface). |

## Presentation Exchange

Data models...

### `PresentationDefinition` 

| Instance Method                                            | Notes |
| ---------------------------------------------------------- | ----- |
| `select_credentials(vc_jwts: &Vec<String>) -> Vec<String>` |       |
