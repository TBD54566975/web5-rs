> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [Core API Reference](#core-api-reference)
  - [Examples](#examples)
  - [Cryptography](#cryptography)
    - [`Jwk`](#jwk)
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
    - [`Service`](#service)
    - [`Resolution`](#resolution)
    - [`DocumentMetadata`](#documentmetadata)
    - [`ResolutionMetadata`](#resolutionmetadata)
    - [Dereference TODO](#dereference-todo)
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
  - [`Jwk`](#jwk-1)
  - [`Alg` (Enum)](#alg-enum)
  - [`LocalKeyManager`](#localkeymanager)
  - [`PortableDid`](#portabledid)
  - [`BearerDid`](#bearerdid)
  - [`VerifiableCredential`](#verifiablecredential-1)
    - [`VcSignOptions`](#vcsignoptions)
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

### `Dsa` (Interface)

| Static Method                                                           | Notes                        |
| ----------------------------------------------------------------------- | ---------------------------- |
| `generate_key(): Jwk`                                                   | 🚧 May need to be a keypair 🚧 |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk).           |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk).           |

### `JwsDsa` (Interface)

| Static Method                                                           | Notes              |
| ----------------------------------------------------------------------- | ------------------ |
| `sign(private_jwk: &Jwk, payload: &[u8]) -> Vec<u8>`                    | See [`Jwk`](#jwk). |
| `verify(public_key: &Jwk, message: &[u8], signature: &[u8]) -> Vec<u8>` | See [`Jwk`](#jwk). |

### `Ed25519`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for Ed25519.

### `Xd25519`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for Xd25519.

### `Secp256k1`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for secp256k1.

### `Secp256r1`

Implements [`Dsa`](#dsa-interface) and [`JwsDsa`](#jwsdsa-interface) for secp256r1.

## Key Managers

### `InMemoryKeyManager` 

Strictly uses Ed25519.

| Instance Method                        | Notes                                                                           |
| :------------------------------------- | :------------------------------------------------------------------------------ |
| `generate_private_key(): Jwk`          | Return [`Jwk`](#jwk) is a public key and MUST NOT contain private key material. |
| `get_dsa(public_jwk: &Jwk) -> Ed25519` |                                                                                 |

## DIDs

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

### `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

### `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

See [`Jwk`](#jwk) for `publicKeyJwk` implementation.

### `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

### `Resolution`

| Property                                  | Notes                                            |
| :---------------------------------------- | :----------------------------------------------- |
| `document: Document`                      | See [`Document`](#document).                     |
| `document_metadata: DocumentMetadata`     | See [`DocumentMetadata`](#documentmetadata).     |
| `resolution_metadata: ResolutionMetadata` | See [`ResolutionMetadata`](#resolutionmetadata). |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: &str) -> Resolution` |       |

### `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

### `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

### Dereference TODO

### `DidJwk`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                        | Notes                              |
| :----------------------------------- | :--------------------------------- |
| `create(public_jwk: &Jwk) -> DidJwk` | See [`Jwk`](#jwk).                 |
| `resolve(uri: &str) -> Resolution`   | See [`Resolution`](#resolution-1). |

### `DidWeb`

| Static Method                      | Notes                              |
| :--------------------------------- | :--------------------------------- |
| `resolve(uri: &str) -> Resolution` | See [`Resolution`](#resolution-1). |

### `DidDht`

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

#### `DidDhtCreateOptions`

| Property | Notes |
| -------- | ----- |

🚧 This is under construction, incomplete 🚧

## Credentials

### `VerifiableCredential`

Data properties conformant to [Verifiable Credential Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                        | Notes                              |
| :------------------------------------- | :--------------------------------- |
| `sign(jws_dsa: &dyn JwsDsa) -> String` | See [`JwsDsa`](#jwsdsa-interface). |


| Static Method                                                     | Notes                              |
| :---------------------------------------------------------------- | :--------------------------------- |
| `verify(jwt: &str, jws_dsa: &dyn JwsDsa) -> VerifiableCredential` | See [`JwsDsa`](#jwsdsa-interface). |

## Presentation Exchange

### `PresentationDefinition` 

| Property                                  | Notes                                      |
| ----------------------------------------- | ------------------------------------------ |
| `id: String`                              |                                            |
| `name: Option<String>`                    |                                            |
| `purpose: Option<String>`                 |                                            |
| `input_descriptors: Vec<InputDescriptor>` | See [`InputDescriptor`](#inputdescriptor). |

| Instance Method                                            | Notes |
| ---------------------------------------------------------- | ----- |
| `select_credentials(vc_jwts: &Vec<String>) -> Vec<String>` |       |

### `InputDescriptor` 

| Property                   | Notes                              |
| -------------------------- | ---------------------------------- |
| `id: String`               |                                    |
| `name: Option<String>`     |                                    |
| `purpose: Option<String>`  |                                    |
| `constraints: Constraints` | See [`Constraints`](#constraints). |

### `Constraints`

| Property             | Notes                  |
| -------------------- | ---------------------- |
| `fields: Vec<Field>` | See [`Field`](#field). |

### `Field`

| Property                         | Notes                              |
| -------------------------------- | ---------------------------------- |
| `id: Option<String>`             |                                    |
| `name: Option<String>`           |                                    |
| `path: Vec<String>`              |                                    |
| `purpose: Option<String>`        |                                    |
| `filter: Option<Filter>`         | See [`Filter`](#filter).           |
| `optional: Optional<bool>`       |                                    |
| `predicate: Option<Optionality>` | See [`Optionality`](#optionality). |

### `Optionality`

| Enum        |
| ----------- |
| `Required`  |
| `Preferred` |

### `Filter`

| Property                        | Notes                    |
| ------------------------------- | ------------------------ |
| `r#type: Option<String>`        |                          |
| `pattern: Option<String>`       |                          |
| `const_value: Option<String>`   |                          |
| `contains: Option<Box<Filter>>` | See [`Filter`](#filter). |

# Convenience API 

## `Jwk`

🚧 Not all languages support the concept of an "extension" 🚧

| Instance Method                  | Notes |
| -------------------------------- | ----- |
| `compute_thumbprint() -> String` |       |

## `Alg` (Enum)

| Value       |
| ----------- |
| `Ed25519`   |
| `Xd25519`   |
| `Secp256k1` |
| `Secp256r1` |

## `LocalKeyManager`

| Instance Method                                  | Notes                                                    |
| ------------------------------------------------ | -------------------------------------------------------- |
| `generate_signing_key(alg: Alg) -> String`       | Return string is equal to the key ID.                    |
| `get_public_key(key_id: String) -> Jwk`          | See [`Jwk`](#jwk).                                       |
| `sign(key_id: String, payload: &[u8]) -> &[u8]`  |                                                          |
| `import_private_key(jwk: Jwk) -> String`         | See [`Jwk`](#jwk). Return string is equal to the key ID. |
| `export_private_key(key_id: String) -> Vec<Jwk>` | See [`Jwk`](#jwk).                                       |

## `PortableDid`

| Property                 | Notes |
| ------------------------ | ----- |
| `uri: String`            |       |
| `private_keys: Vec<Jwk>` |       |
| `document: Document`     |       |

## `BearerDid`

| Property                       | Notes |
| ------------------------------ | ----- |
| `did: Did`                     |       |
| `document: Document`           |       |
| `key_manager: LocalKeyManager` |       |

| Static Method                                  | Notes |
| ---------------------------------------------- | ----- |
| `from_portable_did(portable_did: PortableDid)` |       |

| Instance Method                                   | Notes |
| ------------------------------------------------- | ----- |
| `to_portable_did() -> PortableDid`                |       |
| `get_signer(vm_selector: VmSelector) -> function` | 🚧 🚧 🚧 |

## `VerifiableCredential` 

🚧 Not all languages support the concept of an "extension" 🚧

| Instance Method                                                 | Notes |
| --------------------------------------------------------------- | ----- |
| `sign(bearer_did: BearerDid, options: VcSignOptions) -> String` |       |

### `VcSignOptions` 

| Property               | Notes |
| ---------------------- | ----- |
| `selector: VmSelector` | 🚧     |
| `typ: String`          |       |

## Examples

🚧 This is under construction, incomplete 🚧