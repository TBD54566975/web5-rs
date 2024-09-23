import { catchWeb5Error } from "../../errors";
import { TypescriptSigner } from "../dsa";
import { Jwk } from "../jwk";
export class TypescriptKeyManager {
    constructor(wasmKeyManager) {
        this.wasmKeyManager = wasmKeyManager;
    }
    importPrivateJwk(privateJwk) {
        try {
            const wasmJwk = this.wasmKeyManager.import_private_jwk(privateJwk.toWasmJwk());
            return Jwk.fromWasmJwk(wasmJwk);
        }
        catch (error) {
            throw catchWeb5Error(error);
        }
    }
    getSigner(publicJwk) {
        try {
            const wasmSigner = this.wasmKeyManager.get_signer(publicJwk.toWasmJwk());
            return new TypescriptSigner(wasmSigner);
        }
        catch (error) {
            throw catchWeb5Error(error);
        }
    }
}
