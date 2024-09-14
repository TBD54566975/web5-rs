package main

/*
#cgo LDFLAGS: -L./target/release -lweb5
#include <stdlib.h>
#include <stdio.h>

char* jwk_compute_thumbprint_from_json(const char* jwk_json);
void jwk_free_string(char* s);

typedef int (*intFunc)();

int bridge(intFunc f) {
    return f();
}

// C function declaration for Rust's bridge_in_rust
extern int bridge_in_rust(intFunc);

extern int fortytwo();
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

// Go equivalent of Rust's Jwk struct
type Jwk struct {
	Alg string `json:"alg,omitempty"`
	Kty string `json:"kty"`
	Crv string `json:"crv"`
	D   string `json:"d,omitempty"`
	X   string `json:"x"`
	Y   string `json:"y,omitempty"`
}

func main() {
	// Create an example Jwk instance
	jwk := Jwk{
		Kty: "EC",
		Crv: "secp256k1",
		X:   "x_value",
		Y:   "y_value",
	}

	// Serialize Jwk struct to JSON
	jwkJson, err := json.Marshal(jwk)
	if err != nil {
		fmt.Println("Error serializing JWK:", err)
		return
	}

	// Convert JSON string to C string
	cJwkJson := C.CString(string(jwkJson))
	defer C.free(unsafe.Pointer(cJwkJson))

	// Call Rust function to compute the thumbprint
	thumbprint := C.jwk_compute_thumbprint_from_json(cJwkJson)
	defer C.jwk_free_string(thumbprint)

	// Print the result from Rust (the thumbprint)
	fmt.Println("Thumbprint:", C.GoString(thumbprint))

	// ---

	// Call the C bridge function with the Go fortytwo function.
	result := C.bridge(C.intFunc(C.fortytwo))

	fmt.Println("Result from C calling Go function:", int(result)) // Output: 42

	// ---

	// Call the Rust function with the Go fortytwo function
	result = C.bridge_in_rust(C.intFunc(C.fortytwo))
	fmt.Println("Result from Rust calling Go function:", int(result)) // Output: 42
}
