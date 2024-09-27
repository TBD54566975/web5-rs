package dsa

import (
	"web5/web5c"
)

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}

type innerSigner struct {
	cSigner *web5c.CSigner
}

func (s *innerSigner) Sign(payload []byte) ([]byte, error) {
	return s.cSigner.Sign(payload)
}

func NewSignerFromCSigner(cSigner *web5c.CSigner) Signer {
	return &innerSigner{cSigner}
}
