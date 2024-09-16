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
	"unsafe"
	"web5/crypto"
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
	jwk := crypto.JWK{
		ALG: "Ed25519",
		KTY: "OKP",
		CRV: "Ed25519",
		D:   "UMxzGsW84I6kS3JkenqYI1gH0GmvxYG2ovI69Vlno8g",
		X:   "EzbXpICojY4ZI2i775GwkkTIbe5nuLL13JbdzUfsO6Q",
		Y:   "",
	}
	signer, _ := NewEd25519Signer(jwk)
	signature, _ := signer.Sign(payload)

	encoded := base64.RawURLEncoding.EncodeToString(signature)
	fmt.Println("Base64 Encoded (from golang):", encoded)
	return payload, nil
}
func PocSignerFromGo(signer Signer) {
	cSigner, id := NewCSigner(signer)
	defer FreeCSigner(id)
	C.poc_signer_from_go(&cSigner)
}

// --

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

func PocSignerFromRust() Signer {
	cSigner := C.poc_signer_from_rust()
	return &innerSigner{cSigner: cSigner}
}

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}
