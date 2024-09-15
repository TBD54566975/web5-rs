package dsa

/*
#cgo LDFLAGS: -L../../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"encoding/base64"
	"fmt"
)

// todo temporary
type InGoSigner struct{}
func (s *InGoSigner) Sign(payload []byte) ([]byte, error) {
	encoded := base64.StdEncoding.EncodeToString(payload)
	fmt.Println("Base64 Encoded (from golang):", encoded)
	return payload, nil
}
func ProofOfConcept(signer Signer) {
	SetSigner(signer)

	cSigner := C.CSigner{
		sign: C.signFunc(C.go_signer_sign),
	}

	C.proof_of_concept(&cSigner)
}
// --

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}
