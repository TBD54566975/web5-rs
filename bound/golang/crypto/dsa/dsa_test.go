package dsa_test

import (
	"encoding/base64"
	"fmt"
	"testing"
	"web5/crypto/dsa"
)

func Test_ProofOfConcept(t *testing.T) {
	signer := dsa.InGoSigner{}
	dsa.ProofOfConcept(&signer)
}

func Test_ProofOfConcept2(t *testing.T) {
	signer := dsa.ProofOfConcept2()
	result, _ := signer.Sign([]byte("Test message"))
	encoded := base64.RawURLEncoding.EncodeToString(result)
	fmt.Println("Base64 Encoded (from rust):", encoded)
}
