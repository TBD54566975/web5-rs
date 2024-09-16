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
	"web5/crypto"
)

type PocGoSigner struct{}

func (s *PocGoSigner) Sign(payload []byte) ([]byte, error) {
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
	cSigner, id := newCSigner(signer)
	defer freeCSigner(id)
	C.poc_signer_from_foreign(&cSigner)
}

func PocSignerFromRust() Signer {
	cSigner := C.poc_signer_from_rust()
	signer := NewSigner(cSigner)
	return signer
}
