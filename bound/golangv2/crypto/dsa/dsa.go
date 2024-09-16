package dsa

import "web5/web5c"

type Signer interface {
	Sign(payload []byte) ([]byte, error)
}

type InnerSigner struct {
	CSigner *web5c.CSigner
}

func (s *InnerSigner) Sign(payload []byte) ([]byte, error) {
	return web5c.CSignerSign(s.CSigner, payload)
}
