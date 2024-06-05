> [!WARNING]
> 🚧 Under Construction 🚧

# RFC-0001 Standard Web5 API Design (APID) Document v0.1.0 <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
- [Detailed Design](#detailed-design)
  - [Role in Web5 Development](#role-in-web5-development)
  - [APID `v0.1.0`](#apid-v010)
- [Drawbacks](#drawbacks)
- [Alternatives](#alternatives)
- [Prior Art](#prior-art)

# Summary

One goal of the Rust implementation of Web5 is to act as a core implementation which is binded to non-Rust (AKA "target") languages. Each programming language has it’s own set of unique idioms, but most modern languages support a generalizable set of concepts. The proposal in RFC-0001 is to codify the Web5 API, henceforth referred to as the ***APID*** (”API Design”), in a language-independent form such that all implementations of Web5 support a standard API.

The scope of RFC-0001 is two-part:

1. The role of the APID in the context of Web5 development
2. The original `v0.1.0` of the [APID](../../API_DESIGN.md)

# Motivation

At the time of RFC-0001 there are six distinct implementations of Web5: [web5-js,](https://github.com/TBD54566975/web5-js) [web5-kt,](https://github.com/TBD54566975/web5-kt) [web5-swift,](https://github.com/TBD54566975/web5-swift) [web5-go,](https://github.com/TBD54566975/web5-go) [web5-dart](https://github.com/TBD54566975/web5-dart), and [web5-rs](https://github.com/TBD54566975/web5-rs). Every implementation offers support for the set of features defined in `v0.1.0` of the APID, but each implementation has a unique design over those features. The lack of a standard design across implementations has caused undesirable consequences. 

The benefits of the APID include, but are not limited to:

- Assist in fostering community involvement by enabling community members to reliably communicate & collaborate across languages; separate languages do no act as a barrier to collaboration.
- Stand as a reliable source of truth for building multi-language developer guides at https://developer.tbd.website.
- Lower the barrier of entry for contributions across implementations by limiting context switching.
- Enable stronger assurances of interoperability across languages.
- Act as an origin for all implementations to source documentation comments (”doc comments”).
- Provide a common design for test vectors to enable robust continuous integration.
- Enable productivity increases by compartmentalizing [bike-shedding](https://en.wikipedia.org/wiki/Law_of_triviality).
- Ensure that undesirable augmentations to the API as a byproduct of constraints from binding technologies are corrected for.
- Serve as a governing document for concrete changes to implementations.

# Detailed Design

## Role in Web5 Development

The common sequence of events to a change or addition to the standard Web5 API would follow:

1. RFC + changes to the APID
2. Consensus among stakeholders to (1)
3. Implementation in Rust
4. Bind the Rust implementation to the target languages
5. Implementation in the target languages with use of the binded code (4)

The APID is codified in a [Custom DSL](../../API_DESIGN.md#custom-dsl) (domain-specific language) syntax. See [Alternatives](#alternatives) below for exploration of alternative solutions. The Custom DSL is not expected to have deterministic assurances via programmatic integrations, the test vectors provide assurances, but instead the Custom DSL is intended to convey a design to a developer in a fast, easy, and accessible manor; the goal of the Custom DSL is for the developer to attend to their target language, not the design codification.

The APID is asserted to act as a floor, or a baseline, of feature support — each implementation may choose to expose publicly facing APIs which are beyond the scope of the APID.

The APID defines semantic & name requirements for the given set of features. Unique language idioms are not within the scope of the APID.

The APID is constructed in a fashion such that the concepts span any target language, so long as that target language supports the [Primitive Concepts](../../API_DESIGN.md#primitive-concepts) and the [High-Level Concepts](../../API_DESIGN.md#high-level-concepts) defined in the APID.  

The APID offers documentation comments, test vectors and examples — all of which are encouraged to be utilized in each implementation.

The APID is versioned in accordance with [semantic versioning](https://semver.org/) for reference; but each implementation will manage a semantic version of its artifact independent of the APID version. It is encouraged to make use of git tags in each implementation to track APID version compatibility.

## APID `v0.1.0` 

`v0.1.0` of the APID is intentionally reduced in scope, relative to existing implementations, to serve as an accessible starting state. Namely, the concepts which are heavily integrated in existing implementations and which are excluded from v0.1.0 are: Bearer DID’s, Key Manager polymorphic base class, JWT, and JWS. The proposal of RFC-0001 is to act as a foundation which can be built-upon, and so it does not make any proposals as to the inclusion or exclusion of the noted absent concepts. Subsequent RFC’s are appropriate means bring forth for these matters. 

🚧 `v0.1.0` of the APID excludes doc comments and test vectors, but both of which should be added in subsequent versions. 🚧

🚧 `v0.1.0` of the APID excludes examples, but examples should be added in subsequent versions. 🚧

`v0.1.0` of the APID makes a first-attempt to define the Custom DSL, but is not to be considered comprehensive, and is expected to change over time.

# Drawbacks

One drawback of the APID is it is considered to be yet another resource to maintain. Considered in tradeoff with the benefits, though, the additional maintenance burden is likely to be worthwhile. 

Another drawback of the APID is the potential for the restriction of creative freedom for implementors in target languages. However, it's important to emphasize the APID acts as a floor and does not dictate the entirety of the API surface for each target language; target language implementors may choose to go above the scope of the APID wherein they're free to explore and discover new creations. Furthermore, the APID is an open source resource and so it is open to proposals from anyone, even on matters of seemingly stylistic preferences.

Another drawback of the APID is the potential for slower process time for changes. If there is a requirement for changes to the API to first funnel through the APID, then implementors will be blocked from implementing changes in their target language. Target implementors, however, are free to implement at their own pace, even in parallel to APID proposals, but cannot be considered to be conformant to the APID until proposals are accepted into the APID. Furthermore, considering the benefits listed above, although process time may be elongated on individual features, as a whole productivity is expected to increase. 

Another drawback of the APID is, considering the requirement of spanning many languages, some commonly used programming patterns, such as generics and variadic function parameters, are excluded from the APID, and such constraints may be considered a sacrifice in the given implementation. However, to rearticulate, the APID is considered to be a floor wherein each target implementation may choose to expose additional, and more expressive or convenient, syntactical solutions for the given features. The APID does, however, make assertions as to the names of features, and so target implementations, even considering the freedom to abstract on top of the APID, are restricted in the domain of available names. 

# Alternatives

One alternative to the APID is for each implementation to ad hoc choose its own API design. However, this seems entirely incomatible with the aim of establishing strong assurances for interoperability. A slightly better alternative would be for each implementation to intend to follow the same design as others, through a weak subjectivity and without a single source of truth codified APID, and this is the strategy we have followed to date. However, this still hasn't proven to be a reliable strategy for ensuring all of the benefits defined above, for example, we have experienced the inconsistency in API design enabling a lack of governance which has led to bugs. 

Rather than designing a Custom DSL, we could codify the design via existing technologies, such as the below list, however none of which convey design elements requirements for the intent of the APID.

- UniFFI UDL: overly constrictive, useful for cross-language bindings, but not for high-level programming concepts. For example, `interface` lacks support for data members or static methods.
- OpenAPI: lacks support for polymorphism.
- GraphQL: lacks support for polymorphism.
- Protobuf: lacks support for polymorphism.

# Prior Art

Not aware of any prior art.