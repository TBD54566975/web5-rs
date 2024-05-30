# Decentralized Identity <!-- omit in toc -->

- [`Did`](#did)
- [`Resolution`](#resolution)
- [Data Models](#data-models)
- [Methods](#methods)
  - [`DidJwk`](#didjwk)
  - [`DidWeb`](#didweb)
  - [`DidDht`](#diddht)
    - [`DidDhtCreateOptions`](#diddhtcreateoptions)

# `Did`

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

# `Resolution`

| Property                                  | Notes                                            |
| :---------------------------------------- | :----------------------------------------------- |
| `document: Document`                      | See [`Document`](#document).                     |
| `document_metadata: DocumentMetadata`     | See [`DocumentMetadata`](#documentmetadata).     |
| `resolution_metadata: ResolutionMetadata` | See [`ResolutionMetadata`](#resolutionmetadata). |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: &str) -> Resolution` |       |

# Data Models

Data models conformant to _W3C Decentralized Identifiers v1.0_ [within the `web5-spec`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md) include the following:

- [`Document`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model)
- [`VerificationMethod`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model)
- [`Service`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model)
- [`DocumentMetadata`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model)
- [`ResolutionMetadata`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model)

# Methods

## `DidJwk`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                        | Notes |
| ------------------------------------ | ----- |
| `create(public_key: &Jwk) -> DidJwk` |       |
| `resolve(uri: &str) -> Resolution`   |       |

## `DidWeb`

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `resolve(uri: &str) -> Resolution` |       |

## `DidDht`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                                                                                | Notes      |
| -------------------------------------------------------------------------------------------- | ---------- |
| `create(signer: &dyn DsaSigner, identity_key: &Jwk, options: DidDhtCreateOptions) -> DidDht` |            |
| `resolve(uri: &str) -> Resolution`                                                           |            |
| `update(...todo)`                                                                            | 🚧 params 🚧 |
| `deactivate(...todo)`                                                                        | 🚧 params 🚧 |

### `DidDhtCreateOptions`

| Property                                                | Notes                 |
| ------------------------------------------------------- | --------------------- |
| `publish: bool`                                         |                       |
| `also_known_as: Option<Vec<String>>`                    |                       |
| `controller: Option<Vec<String>>`                       |                       |
| `service: Option<Vec<Service>>`                         |                       |
| `registered_type: Option<Vec<RegisteredDidType>>`       | 🚧 `RegisteredDidType` |
| `verification_methods: Option<Vec<VerificationMethod>>` |                       |
