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
      - [`JwsVerifier` (Interface)](#jwsverifier-interface)
  - [Key Management](#key-management)
      - [`KeyManager` (Interface)](#keymanager-interface)
      - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [DIDs](#dids)
      - [`BearerDid`](#bearerdid)
      - [`Identifier`](#identifier)
    - [Data Model](#data-model)
      - [`Document`](#document)
      - [`VerificationMethod`](#verificationmethod)
      - [`Service`](#service)
    - [Resolution](#resolution)
      - [`Resolution`](#resolution-1)
      - [`DocumentMetadata`](#documentmetadata)
      - [`ResolutionMetadata`](#resolutionmetadata)
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

| Instance Method                | Notes |
| ------------------------------ | ----- |
| `compute_thumbprint(): string` |       |

#### `JwsSigner` (Interface)

| Instance Method                 | Notes |
| ------------------------------- | ----- |
| `sign(payload: []byte): []byte` |       |

#### `JwsVerifier` (Interface)

| Instance Method                              | Notes |
| -------------------------------------------- | ----- |
| `verify(message: []byte, signature: []byte)` |       |

## Key Management

#### `KeyManager` (Interface)

| Instance Method                       | Notes |
| ------------------------------------- | ----- |
| `get_jws_signer(jwk: Jwk): JwsSigner` |       |

#### `InMemoryKeyManager`

Implementation of `KeyManager` which stores key material in-memory.

Uses Ed25519 for the private key material & for the implementation of the return value for `get_jws_signer()`.

| Constructor                        | Notes                 |
| ---------------------------------- | --------------------- |
| `constructor()`                    |                       |
| `constructor(private_keys: []Jwk)` | For import use cases. |

| Instance Method               | Notes                                                                   |
| ----------------------------- | ----------------------------------------------------------------------- |
| `generate_private_key(): Jwk` | Return `Jwk` is a public key and MUST NOT contain private key material. |

## DIDs

#### `BearerDid`

| Property                  | Notes |
| ------------------------- | ----- |
| `identifier: Identifier`  |       |
| `document: Document`      |       |
| `key_manager: KeyManager` |       |

| Instance Method                       | Notes                                                                  |
| ------------------------------------- | ---------------------------------------------------------------------- |
| `get_default_jws_signer(): JwsSigner` | Returns the `JwsSigner` associated with the first Verification Method. |

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

#### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

#### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

#### `Service`

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

### Methods

#### `DidJwk`

| Static Method                                          | Notes |
| ------------------------------------------------------ | ----- |
| `create(key_manager: KeyManager, jwk: Jwk): BearerDid` |       |
| `resolve(uri): Resolution`                             |       |

#### `DidWeb`

| Static Method              | Notes |
| -------------------------- | ----- |
| `resolve(uri): Resolution` |       |

#### `DidDht`

🚧 This is under construction, incomplete 🚧

| Function                                     | Notes                                   |
| -------------------------------------------- | --------------------------------------- |
| `create(key_manager: KeyManager): BearerDid` | TODO need to enable more for the inputs |
| `update()`                                   | TODO                                    |
| `resolve(uri): Resolution`                   |                                         |

## Credentials

#### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                       | Notes |
| ------------------------------------- | ----- |
| `sign(jws_signer: JwsSigner): string` |       |

| Static Method                                                                        | Notes |
| ------------------------------------------------------------------------------------ | ----- |
| `verify(jwt: string): VerifiableCredential`                                          |       |
| `verify_with_verifier(jwt: string, jws_verifier: JwsVerifier): VerifiableCredential` |       |