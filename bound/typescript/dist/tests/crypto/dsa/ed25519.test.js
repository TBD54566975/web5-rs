import { expect } from "chai";
import { Ed25519Generator, Ed25519Signer } from "../../../src/crypto/dsa/ed25519";
import base64url from "base64url";
import { Web5Error } from "../../../src/errors";
describe("Ed25519Generator class", () => {
    it("should set alg to Ed25519", () => {
        const jwk = Ed25519Generator.generate();
        expect(jwk.alg).to.equal("Ed25519");
    });
    it("should set kty to OKP", () => {
        const jwk = Ed25519Generator.generate();
        expect(jwk.kty).to.equal("OKP");
    });
    it("should set crv to Ed25519", () => {
        const jwk = Ed25519Generator.generate();
        expect(jwk.crv).to.equal("Ed25519");
    });
    it("should set the public key (x) with the correct length", () => {
        const jwk = Ed25519Generator.generate();
        const publicKeyBytes = base64url.toBuffer(jwk.x);
        expect(publicKeyBytes.length).to.equal(32);
    });
    it("should set the private key (d) with the correct length", () => {
        const jwk = Ed25519Generator.generate();
        const privateKey = jwk.d;
        expect(privateKey).to.not.be.undefined;
        const privateKeyBytes = base64url.toBuffer(privateKey);
        expect(privateKeyBytes.length).to.equal(32);
    });
});
describe("Ed25519Signer class", () => {
    it("should sign with a valid key", async () => {
        const jwk = Ed25519Generator.generate();
        const signer = new Ed25519Signer(jwk);
        const message = new TextEncoder().encode("Test message");
        const signature = signer.sign(message);
        expect(signature).to.be.instanceOf(Uint8Array);
        expect(signature.length).to.equal(64);
    });
    it("should fail to sign with an invalid private key", async () => {
        const jwk = Ed25519Generator.generate();
        jwk.d = base64url.encode("invalid_d_key");
        const signer = new Ed25519Signer(jwk);
        const message = new TextEncoder().encode("Test message");
        try {
            signer.sign(message);
        }
        catch (error) {
            expect(error instanceof Web5Error).to.equal(true);
            expect(error.variant).to.equal("Crypto");
            expect(error.message).to.include("cryptography error invalid private key length");
        }
    });
    it("should fail to sign with a missing private key", async () => {
        // Generate a valid JWK but remove the private key (d)
        const jwk = Ed25519Generator.generate();
        jwk.d = undefined; // Remove the private key
        const signer = new Ed25519Signer(jwk);
        const message = new TextEncoder().encode("Test message");
        try {
            signer.sign(message);
        }
        catch (error) {
            expect(error instanceof Web5Error).to.equal(true);
            expect(error.variant).to.equal("Crypto");
            expect(error.message).to.equal("cryptography error private key material must be set");
        }
    });
});
