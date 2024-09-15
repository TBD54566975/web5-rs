package crypto

/*
#cgo LDFLAGS: -L../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../bindings/web5_c/web5_c.h"
*/
import "C"
import "unsafe"

type JWK struct {
	ALG string `json:"alg,omitempty"`
	KTY string `json:"kty,omitempty"`
	CRV string `json:"crv,omitempty"`
	D   string `json:"d,omitempty"`
	X   string `json:"x,omitempty"`
	Y   string `json:"y,omitempty"`
}

func (j JWK) ToCJwk() *C.CJwk {
	return &C.CJwk{
		alg: C.CString(j.ALG),
		kty: C.CString(j.KTY),
		crv: C.CString(j.CRV),
		d:   C.CString(j.D),
		x:   C.CString(j.X),
		y:   C.CString(j.Y),
	}
}

func (j JWK) FreeCJwk(cJwk *C.CJwk) {
	C.free(unsafe.Pointer(cJwk.alg))
	C.free(unsafe.Pointer(cJwk.kty))
	C.free(unsafe.Pointer(cJwk.crv))
	C.free(unsafe.Pointer(cJwk.d))
	C.free(unsafe.Pointer(cJwk.x))
	C.free(unsafe.Pointer(cJwk.y))
}

func (j JWK) ComputeThumbprint() (string, error) {
	cJwk := j.ToCJwk()
	defer j.FreeCJwk(cJwk)

	cThumbprint := C.jwk_compute_thumbprint(cJwk)
	if cThumbprint == nil {
		return "", nil
	}
	defer C.free_string(cThumbprint)

	thumbprint := C.GoString(cThumbprint)
	return thumbprint, nil
}
