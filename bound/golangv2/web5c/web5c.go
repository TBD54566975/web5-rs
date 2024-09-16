package web5c

/*
#cgo LDFLAGS: -L../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"errors"
	"runtime"
	"unsafe"
)

type CJwk C.CJwk

func NewCJwk(alg, kty, crv, d, x, y string) *CJwk {
	cJwk := &C.CJwk{
		alg: C.CString(alg),
		kty: C.CString(kty),
		crv: C.CString(crv),
		d:   C.CString(d),
		x:   C.CString(x),
		y:   C.CString(y),
	}

	return (*CJwk)(cJwk)
}

func FreeCJwk(cJwk *CJwk) {
	C.free(unsafe.Pointer(cJwk.alg))
	C.free(unsafe.Pointer(cJwk.kty))
	C.free(unsafe.Pointer(cJwk.crv))
	C.free(unsafe.Pointer(cJwk.d))
	C.free(unsafe.Pointer(cJwk.x))
	C.free(unsafe.Pointer(cJwk.y))
}

func CJwkComputeThumbprint(jwk *CJwk) string {
	cThumbprint := C.jwk_compute_thumbprint((*C.CJwk)(unsafe.Pointer(jwk)))
	defer C.free_string(cThumbprint)
	return C.GoString(cThumbprint)
}

/** --- */

type CEd25519Signer C.CEd25519Signer

func NewCEd25519Signer(cJwk *CJwk) (*CEd25519Signer, error) {
	cSigner := C.ed25519_signer_new((*C.CJwk)(unsafe.Pointer(cJwk)))
	if cSigner == nil {
		return nil, errors.New("failed to create Ed25519Signer")
	}

	runtime.SetFinalizer(cSigner, func(s *C.CEd25519Signer) {
		if s != nil {
			C.ed25519_signer_free(s)
			s = nil
		}
	})

	return (*CEd25519Signer)(cSigner), nil
}

func CEd25519SignerSign(signer *CEd25519Signer, payload []byte) ([]byte, error) {
	cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
	var cSigLen C.size_t

	cSigner := (*C.CEd25519Signer)(signer)

	cSignature := C.ed25519_signer_sign(cSigner, cPayload, C.size_t(len(payload)), &cSigLen)
	if cSignature == nil {
		return nil, errors.New("failed to sign payload")
	}
	defer C.free(unsafe.Pointer(cSignature))

	signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(cSigLen))
	return signature, nil
}
