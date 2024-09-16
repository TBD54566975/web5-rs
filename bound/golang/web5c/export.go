package web5c

/*
#include <stdlib.h>
*/
import "C"
import (
	"unsafe"
)

//export foreign_signer_sign
func foreign_signer_sign(signer_id C.int, payload *C.uchar, payload_len C.size_t, out_len *C.size_t) *C.uchar {
	mu.Lock()
	signFunc, exists := signerRegistry[int(signer_id)]
	mu.Unlock()

	if !exists {
		return nil
	}

	goPayload := C.GoBytes(unsafe.Pointer(payload), C.int(payload_len))

	result, _ := signFunc(goPayload)

	*out_len = C.size_t(len(result))

	cResult := C.CBytes(result)
	return (*C.uchar)(cResult)
}
