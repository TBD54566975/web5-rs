> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [Language-Agnostic Concepts](#language-agnostic-concepts)
- [Examples](#examples)
  - [Create a DID](#create-a-did)
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
        - [Examples](#examples-1)
      - [`DidWeb`](#didweb)
      - [`DidDht`](#diddht)
  - [Credentials](#credentials)
      - [`VerifiableCredential`](#verifiablecredential)
        - [Examples](#examples-2)
          - [Create A `did:jwk`, Create A VC, And Sign It](#create-a-didjwk-create-a-vc-and-sign-it)

# Language-Agnostic Concepts

TODO

# Examples

## Create a DID

```rust
let key_manager = InMemoryKeyManager::new();
let public_jwk = key_manager.generate_private_key()?;
let bearer_did = DidJwk::create(key_manager, public_jwk)?;
let jwt = Jwt::sign(
  bearer_did.get_default_jws_signer(), 
  JwtClaims { iss: bearer_did.identifier.uri }
);
println!(jwt);
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
let vcjwt = verifiable_credential.sign(bearer_did.get_default_jws_signer());
println!(vcjwt);
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
| `resolve(uri: string): Resolution`                     |       |

##### Examples

Create a `did:jwk`:
```rust
let key_manager = InMemoryKeyManager::new();
let public_jwk = key_manager.generate_private_key().unwrap();
let bearer_did = DidJwk::create(key_manager, public_jwk).unwrap();
println!(bearer_did.identifier.uri);
```

Resolve a `did:jwk`:
```rust
let uri = "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwidXNlIjoic2lnIiwiYWxnIjoiRWREU0EiLCJraWQiOiJKUVYzQ0VaQ3BWWnBCWmQ0N0EzLWllTUM1T1BvOHJ5QlQ5cHdLX3NDLUtBIiwieCI6IlUzWXNDNjFJZnBxRjlqUHNRX01UMDBFTTRBQXVHYms0SDN1VVZRczBFelEifQ";
let resolution = DidJwk::resolve(uri).await.unwrap();
println!(resolution.document.id);
```

Instantiate an existing `did:jwk`:
```rust
let private_jwk_json_string = "{\"kty\":\"OKP\",\"crv\":\"Ed25519\",\"use\":\"sig\",\"alg\":\"EdDSA\",\"kid\":\"JQV3CEZCpVZpBZd47A3-ieMC5OPo8ryBT9pwK_sC-KA\",\"d\":\"8L5Y7M4ZNc9Jy5IooJNFaRGatXHZzRRXxGsVidrAsfE\",\"x\":\"U3YsC61IfpqF9jPsQ_MT00EM4AAuGbk4H3uUVQs0EzQ\"}";
let private_jwk = serde_json::from_str::<Jwk>(private_jwk_json_string).unwrap();
let key_manager = InMemoryKeyManager::from_private_jwks(vec![private_jwk]);

let uri = "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwidXNlIjoic2lnIiwiYWxnIjoiRWREU0EiLCJraWQiOiJKUVYzQ0VaQ3BWWnBCWmQ0N0EzLWllTUM1T1BvOHJ5QlQ5cHdLX3NDLUtBIiwieCI6IlUzWXNDNjFJZnBxRjlqUHNRX01UMDBFTTRBQXVHYms0SDN1VVZRczBFelEifQ";
let resolution = DidJwk::resolve(uri).await.unwrap();

let identifier = Identifier::new(uri);

let bearer_did = BearerDid {
  identifier,
  document: resolution.document,
  key_manager
};
```

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

##### Examples

###### Create A `did:jwk`, Create A VC, And Sign It

```rust
let key_manager = InMemoryKeyManager::new();
let public_jwk = key_manager.generate_private_key().unwrap();
let bearer_did = DidJwk::create(key_manager, public_jwk).unwrap();

let verifiable_credential = VerifiableCredential {
  // todo consider convenience function
};

let vcjwt = verifiable_credential.sign(bearer_did.get_default_jws_signer()).unwrap;
println(vcjwt);
```