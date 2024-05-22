> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

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
        - [Examples](#examples)
          - [Create a `did:jwk`](#create-a-didjwk)
          - [Resolve a `did:jwk`](#resolve-a-didjwk)
          - [Reinstantiate an Existing `did:jwk`](#reinstantiate-an-existing-didjwk)
      - [`DidWeb`](#didweb)
      - [`DidDht`](#diddht)
        - [`DidDhtCreateOptions`](#diddhtcreateoptions)
  - [Credentials](#credentials)
      - [`VerifiableCredential`](#verifiablecredential)
        - [Examples](#examples-1)
          - [Create a `did:jwk`, Create a VC, and Sign it](#create-a-didjwk-create-a-vc-and-sign-it)
    - [Presentation Exchange](#presentation-exchange)
      - [`PresentationDefinition`](#presentationdefinition)
      - [`InputDescriptor`](#inputdescriptor)
      - [`Constraints`](#constraints)
      - [`Field`](#field)
      - [`Optionality`](#optionality)
      - [`Filter`](#filter)
- [Examples](#examples-2)
  - [Bring-Your-Own Key Manager \& Cryptography, Sign a VC-JWT, and Verify it](#bring-your-own-key-manager--cryptography-sign-a-vc-jwt-and-verify-it)

# API Reference

## JOSE

#### `Jwk`

🚧 Consider constraining in [`web5-spec`](https://github.com/TBD54566975/web5-spec) 🚧

| Property            | Notes |
| :------------------ | :---- |
| `alg: String`       |       |
| `kty: String`       |       |
| `crv: String`       |       |
| `d: Option<String>` |       |
| `x: String`         |       |
| `y: Option<String>` |       |

#### `JwsSigner` (Interface)

| Instance Method                 | Notes |
| :------------------------------ | :---- |
| `sign(payload: &[u8]): Vec<u8>` |       |

#### `JwsVerifier` (Interface)

| Instance Method                            | Notes |
| :----------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8])` |       |

## Key Management

#### `KeyManager` (Interface)

| Instance Method                              | Notes                                                                                                           |
| :------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `get_jws_signer(jwk: &Jwk): dyn JwsSigner`   | `jwk` input is equal to the public key represented as a [`Jwk`](#jwk). See [`JwsSigner`](#jwssigner-interface). |
| `sign(jwk: &Jwk, payload: &[u8]) -> Vec<u8>` | `jwk` input is equal to the public key represented as a [`Jwk`](#jwk).                                          |

#### `InMemoryKeyManager`

Implementation of [`KeyManager`](#keymanager-interface) which stores key material in-memory.

Strictly uses Ed25519. Internalize implementations for [`JwsSigner`](#jwssigner-interface) (for return value of `get_jws_signer()` from [`KeyManager`](#keymanager-interface)).

| Static Method                                                      | Notes                                    |
| :----------------------------------------------------------------- | :--------------------------------------- |
| `new() -> InMemoryKeyManager`                                      |                                          |
| `from_private_jwks(private_keys: &Vec<Jwk>) -> InMemoryKeyManager` | For import use cases. See [`Jwk`](#jwk). |

| Instance Method               | Notes                                                                           |
| :---------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk` | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |

## DIDs

#### `BearerDid`

| Property                  | Notes |
| :------------------------ | :---- |
| `identifier: Identifier`  |       |
| `document: Document`      |       |
| `key_manager: KeyManager` |       |

| Instance Method                             | Notes                                                                                          |
| :------------------------------------------ | :--------------------------------------------------------------------------------------------- |
| `get_default_jws_signer() -> dyn JwsSigner` | Returns the [`JwsSigner`](#jwssigner-interface) associated with the first Verification Method. |

#### `Identifier`

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

| Static Method                    | Notes |
| :------------------------------- | :---- |
| `parse(uri: &str) -> Identifier` |       |

### Data Model

#### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

#### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

See [`Jwk`](#jwk) for `publicKeyJwk` implementation.

#### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

### Resolution

#### `Resolution`

| Property                                  | Notes                                            |
| :---------------------------------------- | :----------------------------------------------- |
| `document: Document`                      | See [`Document`](#document).                     |
| `document_metadata: DocumentMetadata`     | See [`DocumentMetadata`](#documentmetadata).     |
| `resolution_metadata: ResolutionMetadata` | See [`ResolutionMetadata`](#resolutionmetadata). |

| Static Method                            | Notes |
| :--------------------------------------- | :---- |
| `async resolve(uri: &str) -> Resolution` |       |

#### `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

#### `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

### Methods

#### `DidJwk`

| Static Method                                                  | Notes                                                                                                                                           |
| :------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `create(key_manager: &dyn KeyManager, jwk: &Jwk) -> BearerDid` | `jwk` input is equal to the public key represented as a [`Jwk`](#jwk). See [`KeyManager`](#keymanager-interface) and [`BearerDid`](#bearerdid). |
| `resolve(uri: &str) -> Resolution`                             | See [`Resolution`](#resolution-1).                                                                                                              |

##### Examples

###### Create a `did:jwk`

```rust
let key_manager = InMemoryKeyManager::new();
let public_jwk = key_manager.generate_private_key().unwrap();
let bearer_did = DidJwk::create(key_manager, public_jwk).unwrap();
println!(bearer_did.identifier.uri);
```

###### Resolve a `did:jwk`

```rust
let uri = "did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwidXNlIjoic2lnIiwiYWxnIjoiRWREU0EiLCJraWQiOiJKUVYzQ0VaQ3BWWnBCWmQ0N0EzLWllTUM1T1BvOHJ5QlQ5cHdLX3NDLUtBIiwieCI6IlUzWXNDNjFJZnBxRjlqUHNRX01UMDBFTTRBQXVHYms0SDN1VVZRczBFelEifQ";
let resolution = DidJwk::resolve(uri).await.unwrap();
println!(resolution.document.id);
```

###### Reinstantiate an Existing `did:jwk`

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

| Static Method                            | Notes                              |
| :--------------------------------------- | :--------------------------------- |
| `async resolve(uri: &str) -> Resolution` | See [`Resolution`](#resolution-1). |

#### `DidDht`

| Function                                                                                     | Notes                                                                                                                                                                                                                                      |
| :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create(key_manager: &dyn KeyManager, jwk: &Jwk, options: DidDhtCreateOptions) -> BearerDid` | `jwk` input is equal to the [Identity Key](https://did-dht.com/#identity-key-pair) represented as a [`Jwk`](#jwk). See [`KeyManager`](#keymanager-interface), [`DidDhtCreateOptions`](#diddhtcreateoptions) and [`BearerDid`](#bearerdid). |
| `update()`                                                                                   | 🚧 This is under construction, incomplete 🚧                                                                                                                                                                                                 |
| `async resolve(uri: &str) -> Resolution`                                                     |                                                                                                                                                                                                                                            |

##### `DidDhtCreateOptions`

| Property | Notes |
| -------- | ----- |

🚧 This is under construction, incomplete 🚧

## Credentials

#### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                              | Notes                                    |
| :------------------------------------------- | :--------------------------------------- |
| `sign(jws_signer: &dyn JwsSigner) -> String` | See [`JwsSigner`](#jwssigner-interface). |

| Static Method                                                                             | Notes                                        |
| :---------------------------------------------------------------------------------------- | :------------------------------------------- |
| `verify(jwt: &str) -> VerifiableCredential`                                               |                                              |
| `verify_with_verifier(jwt: &str, jws_verifier: &dyn JwsVerifier) -> VerifiableCredential` | See [`JwsVerifier`](#jwsverifier-interface). |

##### Examples

###### Create a `did:jwk`, Create a VC, and Sign it

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

### Presentation Exchange

#### `PresentationDefinition` 

| Property                                  | Notes                                      |
| ----------------------------------------- | ------------------------------------------ |
| `id: String`                              |                                            |
| `name: Option<String>`                    |                                            |
| `purpose: Option<String>`                 |                                            |
| `input_descriptors: Vec<InputDescriptor>` | See [`InputDescriptor`](#inputdescriptor). |

| Instance Method                                            | Notes |
| ---------------------------------------------------------- | ----- |
| `select_credentials(vc_jwts: &Vec<String>) -> Vec<String>` |       |

#### `InputDescriptor` 

| Property                   | Notes                              |
| -------------------------- | ---------------------------------- |
| `id: String`               |                                    |
| `name: Option<String>`     |                                    |
| `purpose: Option<String>`  |                                    |
| `constraints: Constraints` | See [`Constraints`](#constraints). |

#### `Constraints`

| Property             | Notes                  |
| -------------------- | ---------------------- |
| `fields: Vec<Field>` | See [`Field`](#field). |

#### `Field`

| Property                         | Notes                              |
| -------------------------------- | ---------------------------------- |
| `id: Option<String>`             |                                    |
| `name: Option<String>`           |                                    |
| `path: Vec<String>`              |                                    |
| `purpose: Option<String>`        |                                    |
| `filter: Option<Filter>`         | See [`Filter`](#filter).           |
| `optional: Optional<bool>`       |                                    |
| `predicate: Option<Optionality>` | See [`Optionality`](#optionality). |

#### `Optionality`

| Enum        |
| ----------- |
| `Required`  |
| `Preferred` |

#### `Filter`

| Property                        | Notes                    |
| ------------------------------- | ------------------------ |
| `r#type: Option<String>`        |                          |
| `pattern: Option<String>`       |                          |
| `const_value: Option<String>`   |                          |
| `contains: Option<Box<Filter>>` | See [`Filter`](#filter). |

# Examples

## Bring-Your-Own Key Manager & Cryptography, Sign a VC-JWT, and Verify it

```rust
println!("todo");
```