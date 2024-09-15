package dsa

/*
#cgo LDFLAGS: -L../../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"errors"
	"runtime"
	"unsafe"
	"web5/crypto"
)

type Ed25519Signer struct {
	signer *C.CEd25519Signer
}

func NewEd25519Signer(jwk crypto.JWK) (*Ed25519Signer, error) {
	cJwk := jwk.ToCJwk()
	defer jwk.FreeCJwk(cJwk)

	cSigner := C.ed25519_signer_new((*C.CJwk)(unsafe.Pointer(cJwk)))
	if cSigner == nil {
		return nil, errors.New("failed to create Ed25519Signer")
	}

	signer := &Ed25519Signer{signer: cSigner}

	runtime.SetFinalizer(signer, func(s *Ed25519Signer) {
		if s.signer != nil {
			C.ed25519_signer_free(s.signer)
			s.signer = nil
		}
	})

	return signer, nil
}

func (s *Ed25519Signer) Sign(payload []byte) ([]byte, error) {
	cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
	var cSigLen C.size_t

	cSignature := C.ed25519_signer_sign(s.signer, cPayload, C.size_t(len(payload)), &cSigLen)
	if cSignature == nil {
		return nil, errors.New("failed to sign payload")
	}
	defer C.free(unsafe.Pointer(cSignature))

	signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(cSigLen))
	return signature, nil
}
