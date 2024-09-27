package web5c

/*
#include <stdlib.h>

#include "../../../bindings/web5_c/web5_c.h"
*/
import "C"
import (
	"unsafe"
)

//export foreign_signer_sign
func foreign_signer_sign(signer_id C.int, payload *C.uchar, payload_len C.size_t, out_len *C.size_t) *C.uchar {
	signerMutex.Lock()
	signer, exists := signerRegistry[int(signer_id)]
	signerMutex.Unlock()

	if !exists {
		return nil
	}

	goPayload := C.GoBytes(unsafe.Pointer(payload), C.int(payload_len))

	result, _ := signer.Sign(goPayload)

	*out_len = C.size_t(len(result))

	cResult := C.CBytes(result)
	return (*C.uchar)(cResult)
}

//export foreign_key_manager_import_private_jwk
func foreign_key_manager_import_private_jwk(manager_id C.int, private_jwk *C.CJwk) *C.CJwk {
	managerMutex.Lock()
	manager, exists := managerRegistry[int(manager_id)]
	managerMutex.Unlock()

	if !exists {
		return nil
	}

	goPrivateJWK := NewCJWKFromCGo(private_jwk)

	publicJWK, err := manager.ImportPrivateJWK(goPrivateJWK)
	if err != nil {
		return nil
	}

	return publicJWK.toCGo()
}

//export foreign_key_manager_get_signer
func foreign_key_manager_get_signer(manager_id C.int, public_jwk *C.CJwk) *C.CSigner {
	managerMutex.Lock()
	manager, exists := managerRegistry[int(manager_id)]
	managerMutex.Unlock()

	if !exists {
		return nil
	}

	goPublicJWK := NewCJWKFromCGo(public_jwk)

	signer, err := manager.GetSigner(goPublicJWK)
	if err != nil {
		return nil
	}

	return signer.toCGo()
}
