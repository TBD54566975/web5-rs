> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [Core API Reference](#core-api-reference)
  - [Examples](#examples)
  - [Cryptography](#cryptography)
      - [`Dsa` (Interface)](#dsa-interface)
      - [`JwsDsa` (Interface)](#jwsdsa-interface)
      - [`Ed25519`](#ed25519)
      - [`Xd25519`](#xd25519)
      - [`Secp256k1`](#secp256k1)
      - [`Secp256r1`](#secp256r1)
  - [Key Managers](#key-managers)
      - [`InMemoryKeyManager`](#inmemorykeymanager)
  - [DIDs](#dids)
      - [`Did`](#did)
      - [`Document`](#document)
      - [`VerificationMethod`](#verificationmethod)
      - [`Jwk`](#jwk)
      - [`Service`](#service)
      - [`Resolution`](#resolution)
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
- [Convenience API](#convenience-api)
      - [`KeyManager` (Interface)](#keymanager-interface)
      - [`LocalKeyManager`](#localkeymanager)
      - [`BearerDid`](#bearerdid)
      - [`VerifiableCredential`](#verifiablecredential-1)
  - [Examples](#examples-1)

# Core API Reference

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

#### `Dsa` (Interface)

| Static Method                                                           | Notes                        |
| ----------------------------------------------------------------------- | ---------------------------- |
| `generate_key(): Jwk`                                                   | 🚧 May need to be a keypair 🚧 |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk).           |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk).           |

#### `JwsDsa` (Interface)

| Static Method                                                           | Notes              |
| ----------------------------------------------------------------------- | ------------------ |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk). |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk). |

#### `Ed25519`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for Ed25519.

#### `Xd25519`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for Xd25519.

#### `Secp256k1`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for secp256k1.

#### `Secp256r1`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for secp256r1.

## Key Managers

#### `InMemoryKeyManager` 

Strictly uses Ed25519.

| Instance Method                        | Notes                                                                           |
| :------------------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk`          | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |
| `get_dsa(public_jwk: &Jwk) -> Ed25519` |                                                                                 |

## DIDs

#### `Did`

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

#### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

#### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

See [`Jwk`](#jwk) for `publicKeyJwk` implementation.

#### `Jwk`

🚧 Consider constraining in [`web5-spec`](https://github.com/TBD54566975/web5-spec) 🚧

| Property            | Notes                                                                                                                                                |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `alg: String`       |                                                                                                                                                      |
| `kty: String`       |                                                                                                                                                      |
| `crv: String`       |                                                                                                                                                      |
| `d: Option<String>` | 🚧 `d` is private key material, consider removing here but how it works into [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) requirements 🚧 |
| `x: String`         |                                                                                                                                                      |
| `y: Option<String>` |                                                                                                                                                      |

#### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

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

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                        | Notes                              |
| :----------------------------------- | :--------------------------------- |
| `create(public_jwk: &Jwk) -> DidJwk` | See [`Jwk`](#jwk).                 |
| `resolve(uri: &str) -> Resolution`   | See [`Resolution`](#resolution-1). |

#### `DidWeb`

| Static Method                      | Notes                              |
| :--------------------------------- | :--------------------------------- |
| `resolve(uri: &str) -> Resolution` | See [`Resolution`](#resolution-1). |

#### `DidDht`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                                                                       | Notes                                                                                                                                                  |
| :---------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `create(dsa: &dyn Dsa, identity_key: &Jwk, options: DidDhtCreateOptions) -> DidDht` | See [Identity Key](https://did-dht.com/#identity-key-pair), [`Dsa`](#dsa-interface), [`Jwk`](#jwk), and [`DidDhtCreateOptions`](#diddhtcreateoptions). |
| `resolve(uri: &str) -> Resolution`                                                  |                                                                                                                                                        |

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

| Instance Method                        | Notes                              |
| :------------------------------------- | :--------------------------------- |
| `sign(jws_dsa: &dyn JwsDsa) -> String` | See [`JwsDsa`](#jwsdsa-interface). |


| Static Method                                                     | Notes                              |
| :---------------------------------------------------------------- | :--------------------------------- |
| `verify(jwt: &str, jws_dsa: &dyn JwsDsa) -> VerifiableCredential` | See [`JwsDsa`](#jwsdsa-interface). |

## Presentation Exchange

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

# Convenience API 

#### `KeyManager` (Interface)

#### `LocalKeyManager`

- generate private key
- get public key
- sign
- import
- export

#### `BearerDid`

- to portable did
- from portable did
- get signer

#### `VerifiableCredential`

- sign with bearer did

## Examples

🚧 This is under construction, incomplete 🚧