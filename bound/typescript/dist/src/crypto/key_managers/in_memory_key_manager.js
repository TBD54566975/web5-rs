import wasm from "../../wasm";
import { catchWeb5Error } from "../../errors";
import { TypescriptKeyManager } from ".";
export class InMemoryKeyManager {
    constructor() {
        try {
            this.keyManager = new TypescriptKeyManager(wasm.new_in_memory_key_manager());
        }
        catch (error) {
            throw catchWeb5Error(error);
        }
    }
    importPrivateJwk(privateJwk) {
        return this.keyManager.importPrivateJwk(privateJwk);
    }
    getSigner(publicJwk) {
        return this.keyManager.getSigner(publicJwk);
    }
}
