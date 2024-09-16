package dsa

/*
#cgo LDFLAGS: -L../../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"fmt"
	"sync"
	"unsafe"
)

var (
	signerRegistry = make(map[int]Signer)
	signerCounter  int
	mu             sync.Mutex
)

func newCSigner(signer Signer) (C.CSigner, int) {
	mu.Lock()
	defer mu.Unlock()

	signerCounter++
	signerRegistry[signerCounter] = signer

	return C.CSigner{
		signer_id: C.int(signerCounter),
		sign:      (C.signFunc)(C.foreign_signer_sign),
	}, signerCounter
}

func freeCSigner(id int) {
	mu.Lock()
	defer mu.Unlock()

	delete(signerRegistry, id)
}

type innerSigner struct {
	cSigner *C.CSigner
}

func (s *innerSigner) Sign(payload []byte) ([]byte, error) {
	cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
	payloadLen := C.size_t(len(payload))

	var cSignatureLen C.size_t

	cSignature := C.call_sign(s.cSigner, cPayload, payloadLen, &cSignatureLen)

	if cSignature == nil {
		return nil, fmt.Errorf("sign failed")
	}
	defer C.free(unsafe.Pointer(cSignature))

	signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(cSignatureLen))
	return signature, nil
}

func NewSigner(cSigner *C.CSigner) Signer {
	return &innerSigner{
		cSigner: cSigner,
	}
}

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}
