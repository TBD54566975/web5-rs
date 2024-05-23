> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [API Reference](#api-reference)
  - [Cryptography](#cryptography)
      - [`JwsSigner` (Interface)](#jwssigner-interface)
      - [`JwsVerifier` (Interface)](#jwsverifier-interface)
      - [`KeySigner` (Interface)](#keysigner-interface)
      - [`InMemoryKeySigner`](#inmemorykeysigner)
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
      - [`DidSigner` (Interface)](#didsigner-interface)
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

# API Reference

## Cryptography

#### `JwsSigner` (Interface)

| Instance Method                 | Notes |
| :------------------------------ | :---- |
| `sign(payload: &[u8]): Vec<u8>` |       |

#### `JwsVerifier` (Interface)

| Instance Method                            | Notes |
| :----------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8])` |       |

#### `KeySigner` (Interface)

| Instance Method                                     | Notes                                                      |
| :-------------------------------------------------- | :--------------------------------------------------------- |
| `get_jws_signer(public_jwk: &Jwk): dyn JwsSigner`   | See [`Jwk`](#jwk) and [`JwsSigner`](#jwssigner-interface). |
| `sign(public_jwk: &Jwk, payload: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk).                                         |

#### `InMemoryKeySigner`

Implements [`KeySigner`](#keysigner-interface).

Strictly uses Ed25519. Internalize implementation of [`JwsSigner`](#jwssigner-interface) (for return value of `get_jws_signer()` from [`KeySigner`](#keysigner-interface)).

| Static Method                                                     | Notes                                    |
| :---------------------------------------------------------------- | :--------------------------------------- |
| `new() -> InMemoryKeySigner`                                      |                                          |
| `from_private_jwks(private_jwks: &Vec<Jwk>) -> InMemoryKeySigner` | For import use cases. See [`Jwk`](#jwk). |

| Instance Method               | Notes                                                                           |
| :---------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk` | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |

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

#### `DidSigner` (Interface)

| Instance Method                             | Notes                                                                                          |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `get_default_jws_signer() -> dyn JwsSigner` | Returns the [`JwsSigner`](#jwssigner-interface) associated with the first Verification Method. |

#### `DidJwk`

Implements [`DidSigner`](#didsigner-interface).

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

Implements [`DidSigner`](#didsigner-interface).

| Property                 | Notes |
| ------------------------ | ----- |
| `identifier: Identifier` |       |
| `document: Document`     |       |

| Static Method                                                                                    | Notes                                                                                                                                                              |
| :----------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create(key_signer: &dyn KeySigner, identity_key: &Jwk, options: DidDhtCreateOptions) -> DidDht` | See [Identity Key](https://did-dht.com/#identity-key-pair), [`KeySigner`](#keysigner-interface), [`Jwk`](#jwk), and [`DidDhtCreateOptions`](#diddhtcreateoptions). |
| `resolve(uri: &str) -> Resolution`                                                               |                                                                                                                                                                    |

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

| Instance Method                              | Notes                                    |
| :------------------------------------------- | :--------------------------------------- |
| `sign(jws_signer: &dyn JwsSigner) -> String` | See [`JwsSigner`](#jwssigner-interface). |

🚧 This is under construction, incomplete 🚧

| Static Method                                                                             | Notes                                        |
| :---------------------------------------------------------------------------------------- | :------------------------------------------- |
| `verify(jwt: &str) -> VerifiableCredential`                                               |                                              |
| `verify_with_verifier(jwt: &str, jws_verifier: &dyn JwsVerifier) -> VerifiableCredential` | See [`JwsVerifier`](#jwsverifier-interface). |

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
