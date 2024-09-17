package web5c

/*
#cgo LDFLAGS: -L../../../target/release -lweb5_c
#include <stdlib.h>
#include "../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"errors"
	"fmt"
	"runtime"
	"sync"
	"unsafe"
)

type CJWK struct {
	ALG string
	KTY string
	CRV string
	D   string
	X   string
	Y   string
}

func (j *CJWK) toCGo() *C.CJwk {
	return &C.CJwk{
		alg: C.CString(j.ALG),
		kty: C.CString(j.KTY),
		crv: C.CString(j.CRV),
		d:   C.CString(j.D),
		x:   C.CString(j.X),
		y:   C.CString(j.Y),
	}
}

func NewCJWKFromCGo(cJWK *C.CJwk) *CJWK {
	return &CJWK{
		ALG: C.GoString(cJWK.alg),
		KTY: C.GoString(cJWK.kty),
		CRV: C.GoString(cJWK.crv),
		D:   C.GoString(cJWK.d),
		X:   C.GoString(cJWK.x),
		Y:   C.GoString(cJWK.y),
	}
}

func (j CJWK) ComputeThumbprint() string {
	cgoJWK := j.toCGo()
	defer C.free(unsafe.Pointer(cgoJWK.alg))
	defer C.free(unsafe.Pointer(cgoJWK.kty))
	defer C.free(unsafe.Pointer(cgoJWK.crv))
	defer C.free(unsafe.Pointer(cgoJWK.d))
	defer C.free(unsafe.Pointer(cgoJWK.x))
	defer C.free(unsafe.Pointer(cgoJWK.y))

	cThumbprint := C.jwk_compute_thumbprint(cgoJWK)
	defer C.free_string(cThumbprint)
	return C.GoString(cThumbprint)
}

/** --- */

type CEd25519Signer struct {
	cgoSigner *C.CEd25519Signer
}

func NewCEd25519Signer(cJWK *CJWK) (*CEd25519Signer, error) {
	cgoJWK := cJWK.toCGo()
	cgoSigner := C.ed25519_signer_new(cgoJWK)
	if cgoSigner == nil {
		return nil, errors.New("failed to create Ed25519Signer")
	}

	cSigner := &CEd25519Signer{cgoSigner: cgoSigner}

	runtime.SetFinalizer(cSigner, func(s *CEd25519Signer) {
		if s.cgoSigner != nil {
			C.ed25519_signer_free(s.cgoSigner)
			s.cgoSigner = nil
		}
	})

	return cSigner, nil
}

func (s *CEd25519Signer) Sign(payload []byte) ([]byte, error) {
	cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
	var cSigLen C.size_t

	cSignature := C.ed25519_signer_sign(s.cgoSigner, cPayload, C.size_t(len(payload)), &cSigLen)
	if cSignature == nil {
		return nil, errors.New("failed to sign payload")
	}
	defer C.free(unsafe.Pointer(cSignature))

	signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(cSigLen))
	return signature, nil
}

/** --- */

type SignFunc func(payload []byte) ([]byte, error)

type CSigner struct {
	ID   int
	Sign SignFunc
}

func (s *CSigner) toCGo() *C.CSigner {
	cSigner := &C.CSigner{
		signer_id: C.int(s.ID),
		sign:      (C.signFunc)(C.foreign_signer_sign),
	}
	return cSigner
}

func NewCSignerFromCGo(cSigner *C.CSigner) *CSigner {
	return &CSigner{
		ID: int(cSigner.signer_id),
		Sign: func(payload []byte) ([]byte, error) {
			var outLen C.size_t
			cPayload := (*C.uchar)(unsafe.Pointer(&payload[0]))
			cSignature := C.call_sign(cSigner, cPayload, C.size_t(len(payload)), &outLen)
			if cSignature == nil {
				return nil, fmt.Errorf("sign failed")
			}
			defer C.free(unsafe.Pointer(cSignature))
			signature := C.GoBytes(unsafe.Pointer(cSignature), C.int(outLen))
			return signature, nil
		},
	}
}

var (
	signerRegistry = make(map[int]*CSigner)
	signerCounter  int
	signerMutex    sync.Mutex
)

func RegisterSigner(signFunc SignFunc) *CSigner {
	signerMutex.Lock()
	defer signerMutex.Unlock()

	signerCounter++

	cSigner := &CSigner{
		ID:   signerCounter,
		Sign: signFunc,
	}

	signerRegistry[signerCounter] = cSigner

	return cSigner
}

func FreeSigner(id int) {
	signerMutex.Lock()
	defer signerMutex.Unlock()

	delete(signerRegistry, id)
}

func POCSignerFromRust() *CSigner {
	cgoSigner := C.poc_signer_from_rust()
	return NewCSignerFromCGo(cgoSigner)
}

func POCSignerFromForeign(cSigner *CSigner) {
	cgoSigner := cSigner.toCGo()
	C.poc_signer_from_foreign(cgoSigner)
}

/** --- */

type CInMemoryKeyManager struct {
	cgoManager *C.CInMemoryKeyManager
}

func NewCInMemoryKeyManager() (*CInMemoryKeyManager, error) {
	cgoManager := C.in_memory_key_manager_new()
	if cgoManager == nil {
		return nil, errors.New("failed to create InMemoryKeyManager")
	}

	manager := &CInMemoryKeyManager{cgoManager}

	runtime.SetFinalizer(manager, func(m *CInMemoryKeyManager) {
		if m.cgoManager != nil {
			C.in_memory_key_manager_free(m.cgoManager)
			m.cgoManager = nil
		}
	})

	return manager, nil
}

func (m *CInMemoryKeyManager) ImportPrivateJwk(cPrivateJWK *CJWK) (*CJWK, error) {
	cgoPrivateJWK := cPrivateJWK.toCGo()
	cgoPublicJWK := C.in_memory_key_manager_import_private_jwk(m.cgoManager, cgoPrivateJWK)
	if cgoPublicJWK == nil {
		return nil, errors.New("failed to import private JWK")
	}

	cPublicJWK := NewCJWKFromCGo(cgoPublicJWK)

	return cPublicJWK, nil
}

func (m *CInMemoryKeyManager) GetSigner(cPublicJWK *CJWK) (*CSigner, error) {
	cgoPublicJWK := cPublicJWK.toCGo()
	cgoSigner := C.in_memory_key_manager_get_signer(m.cgoManager, cgoPublicJWK)
	if cgoSigner == nil {
		return nil, errors.New("failed to retrieve signer")
	}

	cSigner := NewCSignerFromCGo(cgoSigner)
	return cSigner, nil
}

/** --- */

type CKeyManager C.CKeyManager

func CKeyManagerImportPrivateJWK(cManager *CKeyManager, cPrivateJWK *CJWK) (*CJWK, error) {
	cgoPrivateJWK := cPrivateJWK.toCGo()
	cgoPublicJWK := C.call_import_private_jwk((*C.CKeyManager)(cManager), cgoPrivateJWK)

	if cgoPublicJWK == nil {
		return nil, fmt.Errorf("failed to import private JWK")
	}

	cPublicJWK := NewCJWKFromCGo(cgoPublicJWK)
	return cPublicJWK, nil
}

func CKeyManagerGetSigner(cManager *CKeyManager, cPublicJWK *CJWK) (*CSigner, error) {
	cgoPublicJWK := cPublicJWK.toCGo()
	cgoSigner := C.call_get_signer((*C.CKeyManager)(cManager), cgoPublicJWK)

	if cgoSigner == nil {
		return nil, fmt.Errorf("failed to get signer")
	}

	cSigner := NewCSignerFromCGo(cgoSigner)

	return cSigner, nil
}

func POCKeyManagerFromRust() *CKeyManager {
	cKeyManager := C.poc_key_manager_from_rust()
	return (*CKeyManager)(cKeyManager)
}
