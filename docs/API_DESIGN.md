> [!WARNING]
> 🚧 🚧 🚧 Under Construction 🚧 🚧 🚧
>
> 🚧 Documentation comments
> 
> 🚧 Test vectors

# Standard Web5 API Design (APID) Document <!-- omit in toc -->

**Last Updated:** May 30, 2024

**Version:** 0.0.1

- [Language-Independent Programmatic Concepts](#language-independent-programmatic-concepts)
  - [Limitations](#limitations)
  - [High-Level Concepts](#high-level-concepts)
  - [Primitive Concepts](#primitive-concepts)
- [Key Material](#key-material)
  - [`Jwk`](#jwk)
  - [`EphemeralKeyManager`](#ephemeralkeymanager)
  - [`PersistentKeyManager`](#persistentkeymanager)
- [Digital Signature Algorithm's (DSA)](#digital-signature-algorithms-dsa)
  - [`Dsa`](#dsa)
  - [`DsaSigner`](#dsasigner)
  - [`DsaVerifier`](#dsaverifier)
  - [`JwsSigner`](#jwssigner)
  - [`JwsVerifier`](#jwsverifier)
  - [`ed25519_generate`](#ed25519_generate)
  - [`Ed25519Signer`](#ed25519signer)
  - [`Ed25519Verifier`](#ed25519verifier)
- [Decentralized Identifier's (DIDs)](#decentralized-identifiers-dids)
  - [`Did`](#did)
  - [`Document`](#document)
    - [`VerificationMethod`](#verificationmethod)
    - [`Service`](#service)
  - [`Resolution`](#resolution)
    - [`ResolutionMetadataError`](#resolutionmetadataerror)
    - [`ResolutionMetadata`](#resolutionmetadata)
    - [`DocumentMetadata`](#documentmetadata)
  - [Methods](#methods)
    - [`DidJwk`](#didjwk)
    - [`DidWeb`](#didweb)
    - [`DidDht`](#diddht)
      - [`DidDhtCreateOptions`](#diddhtcreateoptions)
- [Verifiable Credential's (VCs)](#verifiable-credentials-vcs)
  - [`NamedIssuer`](#namedissuer)
  - [`VerifiableCredential`](#verifiablecredential)
- [Presentation Exchange (PEX)](#presentation-exchange-pex)
  - [`PresentationDefinition`](#presentationdefinition)
  - [`InputDescriptor`](#inputdescriptor)
  - [`Constraints`](#constraints)
  - [`Field`](#field)
  - [`Optionality`](#optionality)
  - [`Filter`](#filter)

# Language-Independent Programmatic Concepts

The design definitions within this design document are intended to span any programming language, so long as the given programming language supports the [High-Level Concepts](#high-level-concepts) and [Primitive Concepts](#primitive-concepts) concepts below. The instantiations of these concepts will be unique to the given idioms for each target programming language.

## Limitations

In order to achieve the goal of defining concrete design definitions which span multiple languages, we must make some sacrifices in our design. Namely, this design excludes ***generics*** and ***variadic function parameters***, because both lack broad support & consistency across expected target programming languages. Implementations may choose to utilize these concepts in their internals, but the publicly accessible Web5 API must exclude these concepts.

The APID does not assert requirements as to the artifact (i.e. npm packages, rust crates, go modules, etc.) makeup of the Web5 API. It is recommended to implement the entirety of Web5 in a single artifact, but each implementation may choose to create multiple artifacts. However, the APID makes no regards for the matter of circular dependencies, and so it may become unviable to implement the APID in it's completeness across multiple artifacts.

## High-Level Concepts

- Polymorphic Base Class
- Object Oriented Class
- Public Data Members
- Constructor
- Instance Method
- Static Method
- Enumeration

> [!NOTE]
> The APID defines *Public Data Members* on instances of *Object Oriented Class* but does not cast assertions on any private data members.

> [!NOTE]
> *Static Method's* may be implemented on a given Object Oriented Class given the implementation language supports the feature.

## Primitive Concepts

| Type              | Representation                          | Description |
| ----------------- | --------------------------------------- | ----------- |
| string            | `String`                                | 🚧           |
| byte              | `Byte`                                  | 🚧           |
| boolean           | `Bool`                                  | 🚧           |
| array             | `[]T`                                   | 🚧           |
| optional/nullable | `T?`                                    | 🚧           |
| hash map          | `Map<T1, T2>`                           | 🚧           |
| function          | `func_name(param1: T1, param2: T2): T3` | 🚧           |
| mixed type        | `T1 \| T2`                              | 🚧           |

# Key Material

> [!NOTE]
> Public & private *key material* are currently strictly represented as [Jwk](#jwk-object-oriented-class), but as the requirement for additional representations (ex: CBOR) present themselves, key material will need to be disintermediated via a polymorphic base class such as `PublicKeyMaterial` (which would expose an instance method for `get_verifier_bytes()`) and `PrivateKeyMaterial` (which would expose instance methods for `to_public_key_material()` and `get_signer_bytes()`), both of which would implement `as_jwk()`, `as_cbor()` and any other concrete representations as instance methods.

## `Jwk`

- **High-Level Concept:** Object Oriented Class.
- **Description:** Partial representation of a [JSON Web Key](https://datatracker.ietf.org/doc/html/rfc7517).

| Member        | Description                                                      |
| ------------- | ---------------------------------------------------------------- |
| `alg: String` | Algorithm used for the key.                                      |
| `kty: String` | Key type, usually indicating the cryptographic algorithm family. |
| `crv: String` | Curve parameter for elliptic curve keys.                         |
| `x: String`   | X coordinate for the elliptic curve point.                       |
| `y: String?`  | Y coordinate for the elliptic curve point.                       |
| `d: String?`  | Private key value (optional, for private keys only).             |

## `EphemeralKeyManager`

- **High-Level Concept:** Object Oriented Class.
- **Description:** an encapsulation of key material maintained in-memory.

| Instance Method                              | Notes |
| :------------------------------------------- | :---- |
| `generate_key_material(): Jwk`               |       |
| `get_signer(public_key: Jwk): Ed25519Signer` |       |
| `import_key(private_key: Jwk)`               |       |

## `PersistentKeyManager` 

🚧 the idea being to offer a "reader" and "writer" (or "getter" and "setter") for the key material to be persisted 🚧

# Digital Signature Algorithm's (DSA)

## `Dsa`

- **High-Level Concept:** Enumeration.
- **Description:** The set of Digital Signature Algorithm's natively supported within this SDK.

| Enumeration |
| :---------- |
| `Ed22519`   |

> [!NOTE]
> We must add support for `Xd25519`, `secp256k1`, and `secp256r1` for [full did:dht conformance](https://did-dht.com/registry/index.html#key-type-index).

## `DsaSigner`

- **High-Level Concept:** Polymorphic Base Class.
- **Description:** Set of functionality required to implement to be a compatible DSA signer.
- **Notes:** Private key material is assumed to be encapsulated.

| Instance Method                     | Notes |
| :---------------------------------- | :---- |
| `dsa_sign(payload: []Byte): []Byte` |       |

## `DsaVerifier`

- **High-Level Concept:** Polymorphic Base Class.
- **Description:** Set of functionality required to implement to be a compatible DSA verifier.
- **Notes:** Public key material is assumed to be encapsulated.

| Instance Method                                          | Notes |
| :------------------------------------------------------- | :---- |
| `dsa_verify(message: []Byte, signature: []Byte): []Byte` |       |

## `JwsSigner`

- **High-Level Concept:** Polymorphic Base Class.
- **Description:** Set of functionality required to implement to be a compatible JWS signer.
- **Notes:** Private key material is assumed to be encapsulated.

| Instance Method                     | Notes |
| :---------------------------------- | :---- |
| `jws_sign(payload: []Byte): []Byte` |       |

## `JwsVerifier`

- **High-Level Concept:** Polymorphic Base Class.
- **Description:** Set of functionality required to implement to be a compatible JWS verifier.
- **Notes:** Public key material is assumed to be encapsulated.

| Instance Method                                          | Notes |
| :------------------------------------------------------- | :---- |
| `jws_verify(message: []Byte, signature: []Byte): []Byte` |       |

## `ed25519_generate`

- **High-Level Concept:** Static Method
- **Description:** generates private key material for Ed25519.

| Static Method             | Notes |
| :------------------------ | :---- |
| `ed25519_generate(): Jwk` |       |

## `Ed25519Signer`

- **High-Level Concept:** Object Oriented Class
- **Description:** Implementation of [`DsaSigner`](#dsasigner) and [`JwsSigner`](#jwssigner) for Ed25519.

| Constructor                     | Notes |
| :------------------------------ | :---- |
| `constructor(private_key: Jwk)` |       |

## `Ed25519Verifier`

- **High-Level Concept:** Object Oriented Class
- **Description:** Implementation of [`DsaVerifier`](#dsaverifier) and [`JwsVerifier`](#jwsverifier) for Ed25519.

| Constructor                    | Notes |
| :----------------------------- | :---- |
| `constructor(public_key: Jwk)` |       |

# Decentralized Identifier's (DIDs)

## `Did`

- **High-Level Concept:** Object Oriented Class
- **Description:** Representation of a [DID Core Identifier](https://www.w3.org/TR/did-core/#identifiers).

| Property                       | Notes |
| :----------------------------- | :---- |
| `uri: String`                  |       |
| `url: String`                  |       |
| `method: String`               |       |
| `id: String`                   |       |
| `params: Map<String, String>?` |       |
| `path: String?`                |       |
| `query: String?`               |       |
| `fragment: String?`            |       |

| Constructor                | Notes |
| -------------------------- | ----- |
| `constructor(uri: String)` |       |

## `Document`

| Property                                   | Notes |
| ------------------------------------------ | ----- |
| `id: String`                               |       |
| `@context: []String?`                      |       |
| `controller: []String?`                    |       |
| `alsoKnownAs: []String?`                   |       |
| `verificationMethod: []VerificationMethod` |       |
| `authentication: []String?`                |       |
| `assertionMethod: []String?`               |       |
| `keyAgreement: []String?`                  |       |
| `capabilityInvocation: []String?`          |       |
| `capabilityDelegation: []String?`          |       |
| `service: []Service?`                      |       |

### `VerificationMethod`

| Property             | Notes |
| -------------------- | ----- |
| `id: String`         |       |
| `type: String`       |       |
| `controller: String` |       |
| `publicKeyJwk: Jwk`  |       |

### `Service`

| Property                    | Notes |
| --------------------------- | ----- |
| `id: String`                |       |
| `type: String`              |       |
| `serviceEndpoint: []String` |       |

## `Resolution`

| Property                                  | Notes |
| :---------------------------------------- | :---- |
| `document: Document`                      |       |
| `document_metadata: DocumentMetadata`     |       |
| `resolution_metadata: ResolutionMetadata` |       |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: String): Resolution` |       |

### `ResolutionMetadataError`

| Enumeration                  |
| ---------------------------- |
| `invalidDid`                 |
| `notFound`                   |
| `representationNotSupported` |
| `methodNotSupported`         |
| `invalidDidDocument`         |
| `invalidDidDocumentLength`   |
| `internalError`              |

### `ResolutionMetadata`

| Property                          | Notes |
| --------------------------------- | ----- |
| `error: ResolutionMetadataError?` |       |

### `DocumentMetadata`

| Property                  | Notes |
| ------------------------- | ----- |
| `created: String?`        |       |
| `updated: String?`        |       |
| `deactivated: Bool?`      |       |
| `nextUpdate: String?`     |       |
| `versionId: String?`      |       |
| `nextVersionId: String?`  |       |
| `equivalentId: []String?` |       |
| `canonicalId: String?`    |       |

## Methods

### `DidJwk`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `create(public_key: Jwk): DidJwk`  |       |
| `resolve(uri: String): Resolution` |       |

### `DidWeb`

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `resolve(uri: String): Resolution` |       |

### `DidDht`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                                                                        | Notes      |
| ------------------------------------------------------------------------------------ | ---------- |
| `create(signer: DsaSigner, identity_key: Jwk, options: DidDhtCreateOptions): DidDht` |            |
| `resolve(uri: String): Resolution`                                                   |            |
| `update(...todo)`                                                                    | 🚧 params 🚧 |
| `deactivate(...todo)`                                                                | 🚧 params 🚧 |

#### `DidDhtCreateOptions`

| Property                                      | Notes                 |
| --------------------------------------------- | --------------------- |
| `publish: Bool`                               |                       |
| `also_known_as: []String?`                    |                       |
| `controller: []String?`                       |                       |
| `service: []Service?`                         |                       |
| `registered_type: []RegisteredDidType?`       | 🚧 `RegisteredDidType` |
| `verification_methods: []VerificationMethod?` |                       |

# Verifiable Credential's (VCs)

## `NamedIssuer`

| Property       | Notes |
| -------------- | ----- |
| `id: String`   |       |
| `name: String` |       |

## `VerifiableCredential`

| Property                        | Notes                      |
| ------------------------------- | -------------------------- |
| `@context: []String`            |                            |
| `id: String`                    |                            |
| `type: []String`                |                            |
| `issuer: String \| NamedIssuer` |                            |
| `issuanceDate: String`          |                            |
| `expirationDate: String?`       |                            |
| `credentialSubject: Object`     | 🚧 `Object` not supported 🚧 |

| Instance Method                       | Notes |
| :------------------------------------ | :---- |
| `sign(jws_signer: JwsSigner): String` |       |

| Static Method                                                                          | Notes                                                                                    |
| :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `verify(vcjwt: String): VerifiableCredential`                                          | Where the natively supported [`Dsa`](#dsa)'s are applied for cryptographic verification. |
| `verify_with_verifier(vcjwt: String, jws_verifier: JwsVerifier): VerifiableCredential` |                                                                                          |

> [!NOTE]
>
> `verify` and `verify_with_verifier` assume `vcjwt` is a compact serialized JWS wherein the `kid` JOSE Header is equal to a DID URI which can be dereferenced to fetch the [`publicKeyJwk`](./did.md#data-models).

# Presentation Exchange (PEX)

## `PresentationDefinition` 

| Property                               | Notes |
| -------------------------------------- | ----- |
| `id: String`                           |       |
| `name: String?`                        |       |
| `purpose: String?`                     |       |
| `input_descriptors: []InputDescriptor` |       |

| Instance Method                                   | Notes |
| ------------------------------------------------- | ----- |
| `select_credentials(vc_jwts: []String): []String` |       |

## `InputDescriptor` 

| Property                   | Notes |
| -------------------------- | ----- |
| `id: String`               |       |
| `name: String?`            |       |
| `purpose: String?`         |       |
| `constraints: Constraints` |       |

## `Constraints`

| Property          | Notes |
| ----------------- | ----- |
| `fields: []Field` |       |

## `Field`

| Property                  | Notes |
| ------------------------- | ----- |
| `id: String?`             |       |
| `name: String?`           |       |
| `path: []String`          |       |
| `purpose: String?`        |       |
| `filter: Filter?`         |       |
| `optional: Bool?`         |       |
| `predicate: Optionality?` |       |

## `Optionality`

| Enumeration |
| ----------- |
| `Required`  |
| `Preferred` |

## `Filter`

| Property               | Notes |
| ---------------------- | ----- |
| `type: String?`        |       |
| `pattern: String?`     |       |
| `const_value: String?` |       |
| `contains: Filter?`    |       |
