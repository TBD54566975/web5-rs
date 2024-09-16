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
	"sync"
)

var (
	signerRegistry = make(map[int]Signer)
	signerCounter  int
	mu             sync.Mutex
)

func NewCSigner(signer Signer) (C.CSigner, int) {
	mu.Lock()
	defer mu.Unlock()

	signerCounter++
	signerRegistry[signerCounter] = signer

	return C.CSigner{
		signer_id: C.int(signerCounter),
		sign:      (C.signFunc)(C.foreign_signer_sign),
	}, signerCounter
}

func FreeCSigner(id int) {
	mu.Lock()
	defer mu.Unlock()

	delete(signerRegistry, id)
}

// todo temporary
type InGoSigner struct{}

func (s *InGoSigner) Sign(payload []byte) ([]byte, error) {
	encoded := base64.StdEncoding.EncodeToString(payload)
	fmt.Println("Base64 Encoded (from golang):", encoded)
	return payload, nil
}
func ProofOfConcept(signer Signer) {
	cSigner, id := NewCSigner(signer)
	defer FreeCSigner(id)
	C.proof_of_concept(&cSigner)
}

// --

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}
