import { expect } from "chai";
import { InMemoryKeyManager } from "../../../src/crypto/key_managers/in_memory_key_manager";
import { Ed25519Generator } from "../../../src/crypto/dsa/ed25519";
import { Jwk } from "../../../src/crypto/jwk";
import { Web5Error } from "../../../src/errors";

describe("InMemoryKeyManager - importPrivateJwk", () => {
  it("should fail if the JWK is not a private key", async () => {
    const keyManager = new InMemoryKeyManager();
    const privateJwk = Ed25519Generator.generate();
    delete privateJwk.d

    try {
      keyManager.importPrivateJwk(privateJwk);
    } catch (error: any) {
      expect(error instanceof Web5Error).to.equal(true);
      expect(error.variant).to.equal("Parameter");
      expect(error.message).to.equal("parameter error private_jwk must be a private key");
    }
  });

  it("should successfully import and return the public JWK", async () => {
    const keyManager = new InMemoryKeyManager();
    const privateJwk = Ed25519Generator.generate();

    const publicJwk = keyManager.importPrivateJwk(privateJwk);
    expect(publicJwk.d).to.be.undefined;
  });
});

describe("InMemoryKeyManager - getSigner", () => {
  it("should fail if the JWK is not a public key", async () => {
    const keyManager = new InMemoryKeyManager();
    const privateJwk = Ed25519Generator.generate();

    try {
      keyManager.getSigner(privateJwk);
    } catch (error: any) {
      expect(error instanceof Web5Error).to.equal(true);
      expect(error.variant).to.equal("Parameter");
      expect(error.message).to.equal("parameter error public_jwk must be a public key");
    }
  });

  it("should fail if the signer is not found", async () => {
    const keyManager = new InMemoryKeyManager();
    const privateJwk = Ed25519Generator.generate()
    delete privateJwk.d

    try {
      keyManager.getSigner(privateJwk);
    } catch (error: any) {
      expect(error instanceof Web5Error).to.equal(true);
      expect(error.variant).to.equal("NotFound");
      expect(error.message).to.equal(
        `not found error signer not found for public_jwk with thumbprint ${privateJwk.computeThumbprint()}`
      );
    }
  });

  it("should return a signer if the public JWK is found", async () => {
    const keyManager = new InMemoryKeyManager();
    const privateJwk = Ed25519Generator.generate();

    let publicJwk = keyManager.importPrivateJwk(privateJwk);

    const signer = keyManager.getSigner(publicJwk);
    expect(signer).to.not.be.undefined;
  });
});