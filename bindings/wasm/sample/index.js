import wasm from "../pkg/web5_wasm.js";

// Create an instance of WasmKeyManager
const keyManager = new wasm.WasmKeyManager();

// Generate a private key
const curve = "Secp256k1"; // or "Ed25519"
const privateKey = keyManager.generate_private_key(curve);
console.log("Generated Private Key:", privateKey);
