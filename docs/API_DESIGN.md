> [!WARNING]
> 🚧 Under Construction 🚧

# Web5 API Design <!-- omit in toc -->

- [API Reference](#api-reference)
  - [Examples](#examples)
  - [Cryptography](#cryptography)
    - [`Jwk`](#jwk)
    - [Digital Signature Algorithm (DSA)](#digital-signature-algorithm-dsa)
      - [`Dsa` (Enumeration)](#dsa-enumeration)
      - [`PrivateKeyMaterial` (Polymorphic Base Class)](#privatekeymaterial-polymorphic-base-class)
      - [`PublicKeyMaterial` (Polymorphic Base Class)](#publickeymaterial-polymorphic-base-class)
      - [`DsaSigner` (Polymorphic Base Class)](#dsasigner-polymorphic-base-class)
      - [`DsaVerifier` (Polymorphic Base Class)](#dsaverifier-polymorphic-base-class)
      - [`JwsSigner` (Polymorphic Base Class)](#jwssigner-polymorphic-base-class)
      - [`JwsVerifier` (Polymorphic Base Class)](#jwsverifier-polymorphic-base-class)
      - [`Ed25519PublicKeyMaterial`](#ed25519publickeymaterial)
      - [`Ed25519PrivateKeyMaterial`](#ed25519privatekeymaterial)
      - [`Ed25519Signer`](#ed25519signer)
      - [`Ed25519Verifier`](#ed25519verifier)
      - [`Xd25519PublicKeyMaterial`](#xd25519publickeymaterial)
      - [`Xd25519PrivateKeyMaterial`](#xd25519privatekeymaterial)
      - [`Xd25519Signer`](#xd25519signer)
      - [`Xd25519Verifier`](#xd25519verifier)
      - [`Secp256k1PublicKeyMaterial`](#secp256k1publickeymaterial)
      - [`Secp256k1PrivateKeyMaterial`](#secp256k1privatekeymaterial)
      - [`Secp256k1Signer`](#secp256k1signer)
      - [`Secp256k1Verifier`](#secp256k1verifier)
      - [`Secp256r1PublicKeyMaterial`](#secp256r1publickeymaterial)
      - [`Secp256r1PrivateKeyMaterial`](#secp256r1privatekeymaterial)
      - [`Secp256r1Signer`](#secp256r1signer)
      - [`Secp256r1Verifier`](#secp256r1verifier)
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
    - [`DidJwk`](#didjwk)
    - [`DidWeb`](#didweb)
    - [`DidDht`](#diddht)
  - [Credentials](#credentials)
    - [`VerifiableCredential`](#verifiablecredential)
  - [Presentation Exchange](#presentation-exchange)
    - [`PresentationDefinition`](#presentationdefinition)
    - [`InputDescriptor`](#inputdescriptor)
    - [`Constraints`](#constraints)
    - [`Field`](#field)
    - [`Optionality`](#optionality)
    - [`Filter`](#filter)
  - [Bearer DID](#bearer-did)
    - [`BearerDid`](#bearerdid)
    - [`LocalKeyManager`](#localkeymanager)
    - [`VerificationMethodSelector`](#verificationmethodselector)

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

| Property            | Notes |
| :------------------ | :---- |
| `alg: String`       |       |
| `kty: String`       |       |
| `crv: String`       |       |
| `x: String`         |       |
| `y: Option<String>` |       |
| `d: Option<String>` |       |

| Instance Method                  | Notes            |
| :------------------------------- | :--------------- |
| `compute_thumbprint() -> String` | 🚧 link to spec 🚧 |

### Digital Signature Algorithm (DSA)

🚧 Consider using polymorphism for key material 🚧

#### `Dsa` (Enumeration)

The set of Digital Signature Algorithm's natively supported within this SDK.

| Value       |
| :---------- |
| `Ed22519`   |
| `Xd22519`   |
| `Secp256k1` |
| `Secp256r1` |

#### `PrivateKeyMaterial` (Polymorphic Base Class)

| Instance Method                                   | Notes |
| :------------------------------------------------ | :---- |
| `as_jwk(): Jwk`                                   |       |
| `get_signer_bytes(): Vec<u8>`                     |       |
| `to_public_key_material(): dyn PublicKeyMaterial` |       |

#### `PublicKeyMaterial` (Polymorphic Base Class)

| Instance Method                 | Notes |
| :------------------------------ | :---- |
| `as_jwk(): Jwk`                 |       |
| `get_verifier_bytes(): Vec<u8>` |       |

#### `DsaSigner` (Polymorphic Base Class)

Private key material must be encapsulated.

| Instance Method                   | Notes |
| :-------------------------------- | :---- |
| `sign(payload: &[u8]) -> Vec<u8>` |       |

#### `DsaVerifier` (Polymorphic Base Class)

Public key material must be encapsulated.

| Instance Method                                       | Notes |
| :---------------------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8]) -> Vec<u8>` |       |

#### `JwsSigner` (Polymorphic Base Class)

Private key material must be encapsulated.

| Instance Method                   | Notes |
| :-------------------------------- | :---- |
| `sign(payload: &[u8]) -> Vec<u8>` |       |

#### `JwsVerifier` (Polymorphic Base Class)

Public key material must be encapsulated.

| Instance Method                                       | Notes |
| :---------------------------------------------------- | :---- |
| `verify(message: &[u8], signature: &[u8]) -> Vec<u8>` |       |

#### `Ed25519PublicKeyMaterial`

Implements [`PublicKeyMaterial`](#publickeymaterial-polymorphic-base-class) for Ed25519.

🚧 define JWK representation 🚧

#### `Ed25519PrivateKeyMaterial`

Implements [`PrivateKeyMaterial`](#privatekeymaterial-polymorphic-base-class) for Ed25519.

🚧 define JWK representation 🚧

#### `Ed25519Signer`

Implements [`DsaSigner`](#dsasigner-polymorphic-base-class) and [`JwsSigner`](#jwssigner-polymorphic-base-class) for Ed25519.

| Property           | Notes |
| :----------------- | :---- |
| `private_key: Jwk` |       |

| Constructor                      | Notes |
| :------------------------------- | :---- |
| `constructor(private_key: &Jwk)` |       |

| Static Method                 | Notes |
| :---------------------------- | :---- |
| `generate_private_key(): Jwk` |       |

#### `Ed25519Verifier`

Implements [`DsaVerifier`](#dsaverifier-polymorphic-base-class) and [`JwsVerifier`](#jwsverifier-polymorphic-base-class) for Ed25519.

| Constructor                     | Notes |
| :------------------------------ | :---- |
| `constructor(public_key: &Jwk)` |       |

#### `Xd25519PublicKeyMaterial`

Implements [`PublicKeyMaterial`](#publickeymaterial-polymorphic-base-class) for Xd25519.

🚧 define JWK representation 🚧

#### `Xd25519PrivateKeyMaterial`

Implements [`PrivateKeyMaterial`](#privatekeymaterial-polymorphic-base-class) for Xd25519.

🚧 define JWK representation 🚧

#### `Xd25519Signer`

Same as [`Ed25519Signer`](#ed25519signer) but for Xd25519.

#### `Xd25519Verifier`

Same as [`Ed25519Verifier`](#ed25519verifier) but for Xd25519.

#### `Secp256k1PublicKeyMaterial`

Implements [`PublicKeyMaterial`](#publickeymaterial-polymorphic-base-class) for Secp256k1.

🚧 define JWK representation 🚧

#### `Secp256k1PrivateKeyMaterial`

Implements [`PrivateKeyMaterial`](#privatekeymaterial-polymorphic-base-class) for Secp256k1.

🚧 define JWK representation 🚧

#### `Secp256k1Signer`

Same as [`Ed25519Signer`](#ed25519signer) but for secp256k1.

#### `Secp256k1Verifier`

Same as [`Ed25519Verifier`](#ed25519verifier) but for secp256k1.

#### `Secp256r1PublicKeyMaterial`

Implements [`PublicKeyMaterial`](#publickeymaterial-polymorphic-base-class) for Secp256r1.

🚧 define JWK representation 🚧

#### `Secp256r1PrivateKeyMaterial`

Implements [`PrivateKeyMaterial`](#privatekeymaterial-polymorphic-base-class) for Secp256r1.

🚧 define JWK representation 🚧

#### `Secp256r1Signer`

Same as [`Ed25519Signer`](#ed25519signer) but for secp256r1.

#### `Secp256r1Verifier`

Same as [`Ed25519Verifier`](#ed25519verifier) but for secp256r1.

## Key Managers

### `InMemoryKeyManager` 

Strictly uses Ed25519.

| Instance Method                                 | Notes                                                                           |
| :---------------------------------------------- | :------------------------------------------------------------------------------ |
| `generate_key_material(): Jwk`                  | Return [`Jwk`](#jwk) is a public key and does not contain private key material. |
| `get_signer(public_key: &Jwk) -> Ed25519Signer` | See [`Ed25519Signer`](#ed25519signer).                                          |

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

### `DidJwk`

🚧

### `DidWeb`

🚧

### `DidDht`

🚧

## Credentials

### `VerifiableCredential`

Data models conformant to *W3C Verifiable Credentials v1.1* [within the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/vc.md#verifiable-credential-data-model).

| Instance Method                              | Notes                                                 |
| :------------------------------------------- | :---------------------------------------------------- |
| `sign(jws_signer: &dyn JwsSigner) -> String` | See [`JwsSigner`](#jwssigner-polymorphic-base-class). |

| Static Method                                                                 | Notes                                                                                                |
| :---------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| `verify_with_defaults(vcjwt: &str) -> VerifiableCredential`                   | Where the natively supported [`Dsa`](#dsa-enumeration)'s are applied for cryptographic verification. |
| `verify(vcjwt: &str, jws_verifier: &dyn JwsVerifier) -> VerifiableCredential` | See [`JwsVerifier`](#jwsverifier-polymorphic-base-class).                                            |

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

## Bearer DID

### `BearerDid`

| Property                       | Notes |
| :----------------------------- | :---- |
| `did: Did`                     |       |
| `document: Document`           |       |
| `key_manager: LocalKeyManager` |       |

| Static Method                         | Notes                                                 |
| :------------------------------------ | :---------------------------------------------------- |
| `from_serialized(serialized: String)` | Where `serialized` is a JSON serialized portable DID. |
| `create_did_jwk(options)`             | 🚧                                                     |
| `create_did_dht(options)`             | 🚧                                                     |

| Instance Method                                 | Notes                                                                 |
| :---------------------------------------------- | :-------------------------------------------------------------------- |
| `to_serialized() -> String`                     | Where the serialized return string is a JSON serialized portable DID. |
| `sign_vcjwt(vc VerifiableCredential) -> String` |                                                                       |

### `LocalKeyManager`

| Instance Method                                   | Notes                                                    |
| :------------------------------------------------ | :------------------------------------------------------- |
| `generate_private_key(alg: Dsa) -> String`        | Return string is equal to the key ID.                    |
| `get_public_key(key_id: String) -> Jwk`           | See [`Jwk`](#jwk).                                       |
| `sign(key_id: String, payload: &[u8]) -> Vec<u8>` |                                                          |
| `import_key(key: Jwk) -> String`                  | See [`Jwk`](#jwk). Return string is equal to the key ID. |
| `export_key(key_id: String) -> Jwk`               | See [`Jwk`](#jwk).                                       |

### `VerificationMethodSelector`

🚧