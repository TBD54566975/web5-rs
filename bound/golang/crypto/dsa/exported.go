package dsa

/*
#include <stdio.h>
#include <stdlib.h>
*/
import "C"
import (
	"sync"
	"unsafe"
)

var (
	signerInstance Signer
	mu             sync.Mutex
)

func SetSigner(s Signer) {
	mu.Lock()
	defer mu.Unlock()
	signerInstance = s
}

//export go_signer_sign
func go_signer_sign(payload *C.uchar, payload_len C.size_t) *C.uchar {
	mu.Lock()
	defer mu.Unlock()

	if signerInstance == nil {
		return nil
	}

	goPayload := C.GoBytes(unsafe.Pointer(payload), C.int(payload_len))

	result, _ := signerInstance.Sign(goPayload)

	cResult := C.CBytes(result)
	return (*C.uchar)(cResult)
}
