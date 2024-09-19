// test/crypto/jwk.test.ts

import { expect } from "chai";
import { JWK, validateJWK } from "../../src/crypto/jwk.js";

describe("JWK Validation", () => {
  it("should return false if JWK is missing kty or alg", () => {
    const invalidJWK: JWK = { kty: "", alg: "" };
    expect(validateJWK(invalidJWK)).to.be.false;
  });

  it("should return true for a valid JWK", () => {
    const validJWK: JWK = { kty: "RSA", alg: "RS256" };
    expect(validateJWK(validJWK)).to.be.true;
  });
});