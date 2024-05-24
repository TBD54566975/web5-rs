> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [API Reference](#api-reference)
  - [Cryptography](#cryptography)
      - [`Dsa` (Interface)](#dsa-interface)
      - [`JoseDsa` (Interface)](#josedsa-interface)
      - [`Ed25519`](#ed25519)
      - [`InMemoryKeyManager`](#inmemorykeymanager)
        - [Examples](#examples)
  - [DIDs](#dids)
      - [`Identifier`](#identifier)
    - [Data Model](#data-model)
      - [`Document`](#document)
      - [`VerificationMethod`](#verificationmethod)
      - [`Jwk`](#jwk)
      - [`Service`](#service)
    - [Resolution](#resolution)
      - [`Resolution`](#resolution-1)
      - [`DocumentMetadata`](#documentmetadata)
      - [`ResolutionMetadata`](#resolutionmetadata)
    - [Methods](#methods)
      - [`DidJwk`](#didjwk)
      - [`DidWeb`](#didweb)
      - [`DidDht`](#diddht)
        - [`DidDhtCreateOptions`](#diddhtcreateoptions)
  - [Credentials](#credentials)
      - [`VerifiableCredential`](#verifiablecredential)
    - [Presentation Exchange](#presentation-exchange)
      - [`PresentationDefinition`](#presentationdefinition)
      - [`InputDescriptor`](#inputdescriptor)
      - [`Constraints`](#constraints)
      - [`Field`](#field)
      - [`Optionality`](#optionality)
      - [`Filter`](#filter)
- [Examples](#examples-1)

# API Reference

## Cryptography

#### `Dsa` (Interface)

- generate
- sign
- verify

#### `JoseDsa` (Interface)

- sign_jws
- verify_jws

#### `Ed25519`

Implements [`Dsa`](#dsa-interface) and [`JoseDsa`](#josedsa-interface) for the Ed25519 curve.

#### `InMemoryKeyManager`

Strictly uses Ed25519.

| Static Method                                                      | Notes                                    |
| :----------------------------------------------------------------- | :--------------------------------------- |
| `new() -> InMemoryKeyManager`                                      |                                          |
| `from_private_jwks(private_jwks: &Vec<Jwk>) -> InMemoryKeyManager` | For import use cases. See [`Jwk`](#jwk). |

| Instance Method               | Notes                                                                           |
| :---------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk` | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |

##### Examples

From existing private key material:
```rust
let private_jwks = serde_json::from_string("[{...your stringified JWK...}]");
let key_manager = InMemoryKeyManager::from_private_jwks(private_jwks);
```

## DIDs

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

#### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

### Resolution

#### `Resolution`

| Property                                  | Notes                                            |
| :---------------------------------------- | :----------------------------------------------- |
| `document: Document`                      | See [`Document`](#document).                     |
| `document_metadata: DocumentMetadata`     | See [`DocumentMetadata`](#documentmetadata).     |
| `resolution_metadata: ResolutionMetadata` | See [`ResolutionMetadata`](#resolutionmetadata). |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: &str) -> Resolution` |       |

#### `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

#### `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

### Methods

#### `DidJwk`

| Property                 | Notes |
| ------------------------ | ----- |
| `identifier: Identifier` |       |
| `document: Document`     |       |

| Static Method                        | Notes                              |
| :----------------------------------- | :--------------------------------- |
| `create(public_jwk: &Jwk) -> DidJwk` | See [`Jwk`](#jwk).                 |
| `resolve(uri: &str) -> Resolution`   | See [`Resolution`](#resolution-1). |

#### `DidWeb`

| Static Method                      | Notes                              |
| :--------------------------------- | :--------------------------------- |
| `resolve(uri: &str) -> Resolution` | See [`Resolution`](#resolution-1). |

#### `DidDht`

| Property                 | Notes |
| ------------------------ | ----- |
| `identifier: Identifier` |       |
| `document: Document`     |       |

| Static Method                                                                                             | Notes                                                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create(dsa: &dyn Dsa, identity_key: &Jwk, options: DidDhtCreateOptions) -> DidDht` | See [Identity Key](https://did-dht.com/#identity-key-pair), [`Dsa`](#dsa-interface), [`Jwk`](#jwk), and [`DidDhtCreateOptions`](#diddhtcreateoptions). |
| `resolve(uri: &str) -> Resolution`                                                                        |                                                                                                                                                                              |

| Instance Method | Notes                                      |
| --------------- | ------------------------------------------ |
| `update()`      | 🚧 This is under construction, incomplete 🚧 |
| `deactivate()`  | 🚧 This is under construction, incomplete 🚧 |

##### `DidDhtCreateOptions`

| Property | Notes |
| -------- | ----- |

🚧 This is under construction, incomplete 🚧

## Credentials

#### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                          | Notes                                |
| :--------------------------------------- | :----------------------------------- |
| `sign(jose_dsa: &dyn JoseDsa) -> String` | See [`JoseDsa`](#josedsa-interface). |


| Static Method                                                       | Notes                                |
| :------------------------------------------------------------------ | :----------------------------------- |
| `verify(jwt: &str, jose_dsa: &dyn JoseDsa) -> VerifiableCredential` | See [`JoseDsa`](#josedsa-interface). |

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

🚧 This is under construction, incomplete 🚧