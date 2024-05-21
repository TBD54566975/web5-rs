> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [Language-Agnostic Concepts](#language-agnostic-concepts)
- [Examples](#examples)
  - [Create a DID and Sign a JWT](#create-a-did-and-sign-a-jwt)
  - [Instantiate an Existing DID, Create a VC, and Sign it](#instantiate-an-existing-did-create-a-vc-and-sign-it)
  - [Verify a VC and Inspect the Credential Subject](#verify-a-vc-and-inspect-the-credential-subject)
  - [Bring-Your-Own Key Manager \& Cryptography, Sign a JWT, and Verify it](#bring-your-own-key-manager--cryptography-sign-a-jwt-and-verify-it)
- [API Reference](#api-reference)
  - [JOSE](#jose)
      - [`Jwk`](#jwk)
      - [`JwsSigner` (Interface)](#jwssigner-interface)
      - [`Ed25519JwsSigner`](#ed25519jwssigner)
      - [`Secp256k1JwsSigner`](#secp256k1jwssigner)
      - [`JwsVerifier` (Interface)](#jwsverifier-interface)
  - [Key Management](#key-management)
      - [`KeyManager` (Interface)](#keymanager-interface)
      - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [DIDs](#dids)
      - [`Identifier`](#identifier)
    - [Data Model](#data-model)
    - [`Document`](#document)
    - [`VerificationMethod`](#verificationmethod)
    - [`Service`](#service)
    - [Resolution](#resolution)
      - [`Resolution`](#resolution-1)
      - [`DocumentMetadata`](#documentmetadata)
      - [`ResolutionMetadata`](#resolutionmetadata)
      - [`BearerDid`](#bearerdid)
    - [Methods](#methods)
      - [`DidJwk`](#didjwk)
      - [`DidWeb`](#didweb)
      - [`DidDht`](#diddht)
  - [Credentials](#credentials)
      - [`VerifiableCredential`](#verifiablecredential)

# Language-Agnostic Concepts

TODO

# Examples

## Create a DID and Sign a JWT

```rust
let bearer_did = DidJwk::create(InMemoryKeyManager::new(), Ed25519PrivateJwkGenerator::new());
let jwt = Jwt::sign(
  bearer_did.get_default_jws_signer(), 
  JwtClaims { iss: bearer_did.identifier.uri }
);
println!(jwt.jws.compact_serialized);
```

## Instantiate an Existing DID, Create a VC, and Sign it

```rust
// existing DID URI & private key material
let did_uri = "did:dht:...";
let private_jwk = serde_json::from_str::<EdDSAPrivateJwk>("{...}")?;

let bearer_did = BearerDid::new(did_uri, InMemoryKeyManager::new(vec![private_jwk])).await?;

let verifiable_credential = VerifiableCredential{
  // todo consider default's for convenience
};
let vcjwt = verifiable_credential.sign_vcjwt(bearer_did.get_default_jws_signer());
// TODO consider default sign() functions return compact_serialized
println!(vcjwt.jws.compact_serialized);
```

## Verify a VC and Inspect the Credential Subject

```rust
println!("todo");
```

## Bring-Your-Own Key Manager & Cryptography, Sign a JWT, and Verify it

```rust
println!("todo");
```

# API Reference

## JOSE

#### `Jwk`

🚧 Consider constraining in `web5-spec` 🚧

| Property      | Notes |
| ------------- | ----- |
| `alg: string` |       |
| `kty: string` |       |
| `d?: string`  |       |
| `x: string`   |       |
| `y?: string`  |       |

#### `JwsSigner` (Interface)

| Instance Method                  | Notes |
| -------------------------------- | ----- |
| `get_algorithm(): string`        |       |
| `get_key_id(): string`           |       |
| `get_signature_length(): number` |       |
| `sign(payload: []byte): []byte`  |       |

#### `Ed25519JwsSigner`

Implementation of `JwsSigner` for Ed25519. 

| Constructor                   | Notes |
| ----------------------------- | ----- |
| `constructor(key_id: string)` |       |

| Property                   | Notes             |
| -------------------------- | ----------------- |
| `algorithm: string`        | MUST be `Ed25519` |
| `signature_length: number` | MUST be `64`      |

#### `Secp256k1JwsSigner`

Implementation of `JwsSigner` for secp256k1. 

| Constructor                   | Notes |
| ----------------------------- | ----- |
| `constructor(key_id: string)` |       |

| Property                   | Notes            |
| -------------------------- | ---------------- |
| `algorithm: string`        | MUST be `ES256K` |
| `signature_length: number` | MUST be `64`     |

#### `JwsVerifier` (Interface)

## Key Management

#### `KeyManager` (Interface)

| Instance Method                                     | Notes |
| --------------------------------------------------- | ----- |
| `get_jws_signer(jwk_thumbprint: string): JwsSigner` |       |

#### `InMemoryKeyManager`

Implementation of `KeyManager` which stores key material in-memory.

Uses Ed25519.

| Constructor                               | Notes                 |
| ----------------------------------------- | --------------------- |
| `constructor()`                           |                       |
| `constructor(private_keys: []PrivateJwk)` | For import use cases. |

## DIDs

#### `Identifier`

| Property                      | Notes |
| ----------------------------- | ----- |
| `uri: string`                 |       |
| `url: string`                 |       |
| `method: string`              |       |
| `id: string`                  |       |
| `params: map<string, string>` |       |
| `path: string`                |       |
| `query: string`               |       |
| `fragment: string`            |       |

| Constructor                | Notes |
| -------------------------- | ----- |
| `constructor(uri: string)` |       |

### Data Model

### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

### Resolution

#### `Resolution`

| Property                                  | Notes |
| ----------------------------------------- | ----- |
| `document: Document`                      |       |
| `document_metadata: DocumentMetadata`     |       |
| `resolution_metadata: ResolutionMetadata` |       |

| Static Method                      | Notes                                                                              |
| ---------------------------------- | ---------------------------------------------------------------------------------- |
| `resolve(uri: string): Resolution` | Resolution may require networked invocation, and if should should be asynchronous. |

#### `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

#### `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the we5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

#### `BearerDid`

| Property                  | Notes |
| ------------------------- | ----- |
| `identifier: Identifier`  |       |
| `document: Document`      |       |
| `key_manager: KeyManager` |       |

| Instance Method                       | Notes                                                                |
| ------------------------------------- | -------------------------------------------------------------------- |
| `get_default_jws_signer(): JwsSigner` | Returns the `JwsSigner` associated to the first Verification Method. |

### Methods

#### `DidJwk`

| Static Method                                                                            | Notes |
| ---------------------------------------------------------------------------------------- | ----- |
| `create(key_manager: KeyManager, private_jwk_generator: PrivateJwkGenerator): BearerDid` |       |
| `resolve(uri): Resolution`                                                               |       |

#### `DidWeb`

| Static Method              | Notes |
| -------------------------- | ----- |
| `resolve(uri): Resolution` |       |

#### `DidDht`

| Function                                     | Notes                                   |
| -------------------------------------------- | --------------------------------------- |
| `create(key_manager: KeyManager): BearerDid` | TODO need to enable more for the inputs |
| `update()`                                   | TODO                                    |
| `resolve(uri): Resolution`                   |                                         |

## Credentials

#### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                          | Notes |
| ---------------------------------------- | ----- |
| `sign(jws_signer: JwsSigner): string`    |       |
| `sign_vcjwt(jws_signer: JwsSigner): Jwt` |       |

| Static Method                               | Notes |
| ------------------------------------------- | ----- |
| `verify(jwt: string): VerifiableCredential` |       |