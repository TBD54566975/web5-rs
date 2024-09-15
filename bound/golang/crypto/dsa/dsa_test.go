package dsa_test

import (
	"testing"
	"web5/crypto/dsa"
)

func Test_ProofOfConcept(t *testing.T) {
	signer := dsa.InGoSigner{}

	dsa.ProofOfConcept(&signer)
}
