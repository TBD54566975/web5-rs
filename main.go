package main

/*
#cgo LDFLAGS: -L./target/release -lweb5
#include <stdlib.h>

char* jwk_compute_thumbprint_from_json(const char* jwk_json);
void jwk_free_string(char* s);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

// Go equivalent of Rust's Jwk struct
type Jwk struct {
    Alg string  `json:"alg,omitempty"`
    Kty string  `json:"kty"`
    Crv string  `json:"crv"`
    D   string  `json:"d,omitempty"`
    X   string  `json:"x"`
    Y   string  `json:"y,omitempty"`
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
}