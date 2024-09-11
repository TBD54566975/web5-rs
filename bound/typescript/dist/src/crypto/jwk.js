import { catchWeb5Error } from "../errors";
import wasm from "../wasm";
export class Jwk {
    static fromWasmJwk(wasmJwk) {
        return new Jwk(wasmJwk.alg, wasmJwk.kty, wasmJwk.crv, wasmJwk.d, wasmJwk.x, wasmJwk.y);
    }
    constructor(alg, kty, crv, d, x, y) {
        this.alg = alg;
        this.kty = kty;
        this.crv = crv;
        this.d = d;
        this.x = x;
        this.y = y;
    }
    toWasmJwk() {
        return new wasm.WasmJwk(this.alg, this.kty, this.crv, this.d, this.x, this.y);
    }
    computeThumbprint() {
        try {
            const wasmJwk = this.toWasmJwk();
            return wasmJwk.compute_thumbprint();
        }
        catch (error) {
            throw catchWeb5Error(error);
        }
    }
}
