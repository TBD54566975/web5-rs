import wasm from "../../wasm";
import { Jwk } from "../jwk";
import { Signer } from "../dsa";
import { catchWeb5Error } from "../../errors";
import { TypescriptKeyManager } from ".";

export class InMemoryKeyManager {
  private keyManager: TypescriptKeyManager;

  constructor() {
    try {
      this.keyManager = new TypescriptKeyManager(wasm.new_in_memory_key_manager());
    } catch (error) {
      throw catchWeb5Error(error);
    }
  }

  importPrivateJwk(privateJwk: Jwk): Jwk {
    return this.keyManager.importPrivateJwk(privateJwk)
  }

  getSigner(publicJwk: Jwk): Signer {
    return this.keyManager.getSigner(publicJwk)
  }
}