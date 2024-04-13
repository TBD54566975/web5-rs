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
});

// Use the library
const manager = lib.local_key_manager_new();

if (!manager.isNull()) {
  const outKeyAlias = ref.alloc(cStringPtr);
  const curve = new FFICurve();
  curve.value = 0;  // 0 for Secp256k1, 1 for Ed25519

  const result = lib.local_key_manager_generate_private_key(manager, curve.ref(), outKeyAlias);  // Pass reference to curve
  if (result) {
    const keyAlias = outKeyAlias.deref().readCString(0);
    console.log('Generated key alias:', keyAlias);

    // Don't forget to free the C string when you're done
    ffi.Library('libc', {'free': ['void', ['pointer']]}).free(outKeyAlias.deref());
  } else {
    console.log('Failed to generate key.');
  }

  // Clean up
  lib.local_key_manager_free(manager);
} else {
  console.log('Failed to create key manager.');
}
