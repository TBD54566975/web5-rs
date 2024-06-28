# RFC-0003 Add `PortableDid` to (APID) Document v1.1.0 <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
- [Detailed Design](#detailed-design)
  - [Known Limitations](#known-limitations)
- [Drawbacks](#drawbacks)
- [Alternatives](#alternatives)
- [Prior Art](#prior-art)

# Summary

Add `PortableDid` to the APID, and add feature support to `BearerDid`'s for instantiation *from* a `PortableDid`.

# Motivation

Development and testing environments need quick and easy accessibility for creating DIDs, persisting & reinstantiating said DIDs, and signing VC-JWTs with those DIDs -- all of which require the management of private key material. While there is support with APID v1.0.0 (& prior) for what's added here, it's considered to be excessibely verbose and therefore inaccessible; `PortableDid`'s streamline the creation and re-instantiation of DIDs into a single line of code. The reduction in friction made available with `PortableDid`'s may seem marginal to those of us already accustomed to the Web5 concepts, but to newcomers the reduction in friction makes all the difference.

Key management across the broad array of potential environments and platforms [is non-uniform](https://github.com/TBD54566975/web5-spec/issues/142#issuecomment-2035411492). While the APID makes it explicitly clear, `PortableDid`'s are not considered safe for production environments, we must recognize due to the lack of support for cryptographic requirements, some production use cases may need to serialize private keys and make use of the `InMemoryKeyManager`.

# Detailed Design

Addition of `PortableDid` which is solely a data class, with no instance members, but does contain a constructor for JSON deserialization.

You'll notice there is no `BearerDid.export()` function, and this is intentional. Serialization of private key material is UNSAFE and therefore should be encouraged against. Creation of DIDs (and the resulting `PortableDid` with the inclusion of serialized private key material) is encouraged to be administered via the `web5` CLI, whereafter the values can be used to instantiate the `BearerDid`. Application use cases, such as an identity wallet, which require user-facing serialization of key material, are therefore required to programmatically serialize their own private key materials.

## Known Limitations

- Known issue wherein the `document` may be out-of-date with the hosted DID Document [`web5-spec` #156](https://github.com/TBD54566975/web5-spec/issues/156).

# Drawbacks

Serialization of private key material may enable accidental security events.

# Alternatives

The alternative would be to require developers to serialize/deserialize their private key material, which would increase friction.

**Creation:**

```pseudocode!
let key_manager = new InMemoryKeyManager()
let private_jwk = Ed25519Generator::generate()
let public_jwk = key_manager.import_jwk(private_jwk)
let did_dht = new DidDht(public_jwk)
let bearer_did = new BearerDid(did_dht.did.uri, key_manager)
// somehow persist the `private_jwk` for later (ex. write to a file)
```

**Reinstantiation:**

```pseudocode!
let did_uri = "did:jwk:..." // the developer would have to persist this as well
let private_jwk = JSON.parse(env.get("PRIVATE_JWK")) // or wherever the private key material was persisted to
let key_manager = new InMemoryKeyManager()
key_manager.import_private_jwk(private_jwk)
let bearer_did = new Bearer(did_uri, key_manager)
```

**With `PortableDid`:**

```shell
$> web5 did create jwk
# copy the output (which is a `PortableDid`)
```

```pseudocode!
let bearer_did = new BearerDid(new PortableDid(env.get("PORTABLE_DID")))
```

# Prior Art

We have implemented the `PortableDid` in all of our prior Web5 SDK implementations.