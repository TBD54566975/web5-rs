# Decentralized Identity <!-- omit in toc -->

🚧 Consider splitting methods out into their own 🚧

Data models conformant to _W3C Decentralized Identifiers v1.0_ [within the `web5-spec`](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md).

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

# `Document`

Data properties conformant to [DID Document Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-data-model).

# `VerificationMethod`

Data properties conformant to [Verification Method Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#verification-method-data-model).

See [`Jwk`](#jwk) for `publicKeyJwk` implementation.

# `Service`

Data properties conformant to [Service Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#service-data-model).

# `Resolution`

| Property                                  | Notes                                            |
| :---------------------------------------- | :----------------------------------------------- |
| `document: Document`                      | See [`Document`](#document).                     |
| `document_metadata: DocumentMetadata`     | See [`DocumentMetadata`](#documentmetadata).     |
| `resolution_metadata: ResolutionMetadata` | See [`ResolutionMetadata`](#resolutionmetadata). |

| Static Method                      | Notes |
| :--------------------------------- | :---- |
| `resolve(uri: &str) -> Resolution` |       |

# `DocumentMetadata`

Data properties conformant to the [DID Document Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-document-metadata-data-model).

# `ResolutionMetadata`

Data properties conformant to [DID Resolution Metadata Data Model in the web5-spec](https://github.com/TBD54566975/web5-spec/blob/main/spec/did.md#did-resolution-metadata-data-model).

# `DidJwk`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                        | Notes |
| ------------------------------------ | ----- |
| `create(public_key: &Jwk) -> DidJwk` |       |
| `resolve(uri: &str) -> Resolution`   |       |

# `DidWeb`

| Property             | Notes |
| -------------------- | ----- |
| `did: Did`           |       |
| `document: Document` |       |

| Static Method                      | Notes |
| ---------------------------------- | ----- |
| `resolve(uri: &str) -> Resolution` |       |

# `DidDht`

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

## `DidDhtCreateOptions`

| Property                                                | Notes                 |
| ------------------------------------------------------- | --------------------- |
| `publish: bool`                                         |                       |
| `also_known_as: Option<Vec<String>>`                    |                       |
| `controller: Option<Vec<String>>`                       |                       |
| `service: Option<Vec<Service>>`                         |                       |
| `registered_type: Option<Vec<RegisteredDidType>>`       | 🚧 `RegisteredDidType` |
| `verification_methods: Option<Vec<VerificationMethod>>` |                       |
