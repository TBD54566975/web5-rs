package dsa

/*
#cgo LDFLAGS: -L../../../../target/release -lweb5_c
#include <stdlib.h>
#include <string.h>
#include "../../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"encoding/base64"
	"fmt"
	"sync"
	"unsafe"
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
	encoded := base64.RawURLEncoding.EncodeToString(payload)
	fmt.Println("Base64 Encoded (from golang):", encoded)
	return payload, nil
}
func ProofOfConcept(signer Signer) {
	cSigner, id := NewCSigner(signer)
	defer FreeCSigner(id)
	C.proof_of_concept(&cSigner)
}

// --

type innerSigner struct {
	cSigner *C.CSigner
}

func (s *innerSigner) Sign(payload []byte) ([]byte, error) {
	cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
	payloadLen := C.size_t(len(payload))

	cSignature := C.call_sign(s.cSigner, cPayload, payloadLen)

	if cSignature == nil {
		return nil, fmt.Errorf("sign failed")
	}
	defer C.free(unsafe.Pointer(cSignature))

	signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(C.strlen((*C.char)(unsafe.Pointer(cSignature)))))
	return signature, nil
}

func ProofOfConcept2() Signer {
	cSigner := C.proof_of_concept_2()
	return &innerSigner{cSigner: cSigner}
}

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}
