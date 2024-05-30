> [!WARNING]
> 🚧 Under Construction 🚧

🚧 Pull Request 🚧

Title: ???

Description: 
- TSC, wrong working group?
- `core/` directory

# RFC-0001 Standard Web5 API Design (APID) Document v0.0.1 <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
- [Detailed Design](#detailed-design)
  - [Role in Web5 Development](#role-in-web5-development)
  - [APID `v0.0.1`](#apid-v001)
- [Drawbacks](#drawbacks)
- [Alternatives](#alternatives)
- [Prior Art](#prior-art)

# Summary

One goal of the Rust implementation of Web5 is to act as a core implementation which is binded to non-Rust (target) languages. Each programming language has it’s own set of unique idioms, but all planned-for languages support a generalizable set of concepts. The proposal in RFC-0001 is to codify the Web5 API in a language-independent form, henceforth referred to as the ***APID*** (”API Design”), such that all implementations of Web5 support a standardized API.

The scope of RFC-0001 is two-part:

1. The role of the APID in the context of Web5 development
2. The original `v0.0.1` of the APID

# Motivation

At the time of RFC-0001 there are six distinct implementations of Web5: [web5-js,](https://github.com/TBD54566975/web5-js) [web5-kt,](https://github.com/TBD54566975/web5-kt) [web5-swift,](https://github.com/TBD54566975/web5-swift) [web5-go,](https://github.com/TBD54566975/web5-go) [web5-dart](https://github.com/TBD54566975/web5-dart), and [web5-rs](https://github.com/TBD54566975/web5-rs). Every implementation offers support for the set of features defined in `v0.0.1` of the APID, but each implementation has a unique design over those features. The lack of a standardized design across implementations has proven to lead to many undesirable consequences. 

The benefits of the APID include, but are not limited to:

- The APID will assist in fostering community involvement such that community members can reliably communicate & collaborate across languages.
- The APID will offer a reliable source of truth for building multi-language developer guides at [https://developer.tbd.website](https://developer.tbd.website/).
- The APID will lower the barrier of entry for contributions across implementations by limiting context switching.
- The APID will offer stronger assurances in interoperability across languages.
- The APID will act as an origin for all implementations to source documentation comments (”doc comments”) and examples.
- The APID will offer a set of test vectors which will decrease the chances of bugs per implementation.
- The APID will enable productivity increases by compartmentalizing [bike-shedding](https://en.wikipedia.org/wiki/Law_of_triviality).
- The APID will ensure that any augmentations to the API as a product of constraints from binding technologies are corrected for.

# Detailed Design

## Role in Web5 Development

The APID is asserted to act as a floor of feature support — each implementation may choose to expose publicly facing APIs which are beyond the scope of the APID.

Semantic or naming changes to the implementation in any language of any feature defined in the APID must be preceded by proposed changes to the APID, and in any case where the changes are substantive, the proposal must be attached to an RFC; proposals to a change in APID must reach consensus prior to implementation.

The APID defines semantic & names for the given set of features. Unique language idioms are not within the scope of the APID.

The APID is constructed in a fashion such that the concepts span any target language, so long as that target language supports polymorphism.  

The APID offers documentation comments, examples, and test vectors — all of which are encouraged to be utilized in each implementation.

The APID is versioned to act as a reference; but each implementation will manage a semantic version independent of the APID version. It is encouraged to make use of git tags in each implementation to track APID version compatibility.

## APID `v0.0.1` 

`v0.0.1` of the APID is intentionally reduced in scope relative to existing implementations, and this is intentional to act as an accessible starting state. Namely, the concepts which are heavily integrated in existing implementations and which are excluded from v0.0.1 are: Bearer DID’s, Key Manager polymorphic base class, JWT, JWS. The proposal of RFC-0001 is to act as a foundation which can be built-upon, and so it does not make any proposals as to the inclusion or exclusion of the noted absent concepts. Subsequent RFC’s are appropriate means bring forth for these matters. 

🚧

# Drawbacks

🚧

# Alternatives

🚧

# Prior Art

🚧