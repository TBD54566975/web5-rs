import { catchWeb5Error } from "../../errors";
export class TypescriptSigner {
    constructor(wasmSigner) {
        this.wasmSigner = wasmSigner;
    }
    sign(payload) {
        try {
            return this.wasmSigner.sign(payload);
        }
        catch (error) {
            throw catchWeb5Error(error);
        }
    }
}
