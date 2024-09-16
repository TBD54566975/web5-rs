package key_managers

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

type InMemoryKeyManager struct {
	manager *C.CInMemoryKeyManager
}

func NewInMemoryKeyManager() (*InMemoryKeyManager, error) {
	cManager := C.in_memory_key_manager_new()
	if cManager == nil {
		return nil, errors.New("failed to create InMemoryKeyManager")
	}

	manager := &InMemoryKeyManager{manager: cManager}

	runtime.SetFinalizer(manager, func(m *InMemoryKeyManager) {
		if m.manager != nil {
			C.in_memory_key_manager_free(m.manager)
			m.manager = nil
		}
	})

	return manager, nil
}

func (m *InMemoryKeyManager) ImportPrivateJwk(jwk crypto.JWK) (*crypto.JWK, error) {
	cJwk := jwk.ToCJwk()
	defer jwk.FreeCJwk(cJwk)

	cPublicJwk := C.in_memory_key_manager_import_private_jwk(m.manager, (*C.CJwk)(unsafe.Pointer(cJwk)))
	if cPublicJwk == nil {
		return nil, errors.New("failed to import private JWK")
	}
	defer C.free(unsafe.Pointer(cPublicJwk))

	publicJwk := &crypto.JWK{
		ALG: C.GoString(cPublicJwk.alg),
		KTY: C.GoString(cPublicJwk.kty),
		CRV: C.GoString(cPublicJwk.crv),
		D:   C.GoString(cPublicJwk.d),
		X:   C.GoString(cPublicJwk.x),
		Y:   C.GoString(cPublicJwk.y),
	}
	return publicJwk, nil
}

// func (m *InMemoryKeyManager) GetSigner(jwk crypto.JWK) (dsa.Signer, error) {
// 	cJwk := jwk.ToCJwk()
// 	defer jwk.FreeCJwk(cJwk)

// 	cSigner := C.in_memory_key_manager_get_signer(m.manager, (*C.CJwk)(unsafe.Pointer(cJwk)))
// 	if cSigner == nil {
// 		return nil, errors.New("failed to retrieve signer")
// 	}

// 	signer := dsa.NewSigner(cSigner)
// 	return signer, nil
// }
