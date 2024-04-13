const ffi = require('ffi-napi');
const ref = require('ref-napi');
const StructType = require('ref-struct-di')(ref);

// Define the FFI types
const voidPtr = ref.refType(ref.types.void);
const cStringPtr = ref.refType(ref.types.CString);

// Define the FFICurve struct
const FFICurve = StructType({
  value: 'int'
});

// Define your library
const lib = ffi.Library('../../../target/debug/libweb5_c.dylib', {
  'local_key_manager_new': ['pointer', []],
  'local_key_manager_free': ['void', ['pointer']],
  'local_key_manager_generate_private_key': ['bool', ['pointer', FFICurve, cStringPtr]],
  'create_did_jwk': ['pointer', ['pointer', 'int']],  // Assuming this returns a pointer to BearerDid
  'free_bearer_did': ['void', ['pointer']],
});

// Use the library to create a key manager, generate a private key, and then create a DID JWK
function manageDIDCreation() {
  const manager = lib.local_key_manager_new();
  if (!manager.isNull()) {
    const outKeyAlias = ref.alloc(cStringPtr);
    const curve = new FFICurve();
    curve.value = 0;  // 0 for Secp256k1, 1 for Ed25519

    const keyGenResult = lib.local_key_manager_generate_private_key(manager, curve.ref(), outKeyAlias);
    if (keyGenResult) {
      const keyAlias = outKeyAlias.deref().readCString(0);
      console.log('Generated key alias:', keyAlias);

      // Create a DID JWK using the generated private key
      const bearerDidPtr = lib.create_did_jwk(manager, curve.value);
      if (!bearerDidPtr.isNull()) {
        console.log('DID JWK created successfully.');

        // Optionally, interact more with the BearerDid instance
        // For demonstration, we just free it here
        lib.free_bearer_did(bearerDidPtr);
      } else {
        console.error('Failed to create DID JWK.');
      }

      // Free the C string when you're done
      ffi.Library('libc', {'free': ['void', ['pointer']]}).free(outKeyAlias.deref());
    } else {
      console.log('Failed to generate key.');
    }

    // Clean up the key manager
    lib.local_key_manager_free(manager);
  } else {
    console.log('Failed to create key manager.');
  }
}

// Execute the function
manageDIDCreation();
