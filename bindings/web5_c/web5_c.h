#ifndef WEB5_C_H
#define WEB5_C_H

#include <stdlib.h>

typedef struct {
	const char* alg;
	const char* kty;
	const char* crv;
	const char* d;
	const char* x;
	const char* y;
} CJwk;

char* jwk_compute_thumbprint(const CJwk* jwk);
void free_string(char* s);

#endif // WEB5_C_H