#ifndef WEB5_C_H
#define WEB5_C_H

#include <stdlib.h>

void free_string(char *s);

/** jwk */
typedef struct
{
	const char *alg;
	const char *kty;
	const char *crv;
	const char *d;
	const char *x;
	const char *y;
} CJwk;
char *jwk_compute_thumbprint(const CJwk *jwk);
/** --- */

/** dsa signer */
typedef unsigned char *(*signFunc)(int signer_id, const unsigned char *payload, size_t payload_len, size_t *out_len);
typedef struct
{
	int signer_id;
	signFunc sign;
} CSigner;
unsigned char *call_sign(CSigner *signer, const unsigned char *payload, size_t payload_len, size_t *out_len);
void poc_signer_from_foreign(const CSigner *signer);

CSigner *new_ed25519_signer(const CJwk *jwk);
/** --- */

/** key managers */
typedef CJwk *(*importFunc)(int manager_id, const CJwk *private_jwk);
typedef CSigner *(*getSignerFunc)(int manager_id, const CJwk *public_jwk);
typedef struct
{
	int manager_id;
	importFunc import_private_jwk;
	getSignerFunc get_signer;
} CKeyManager;
CJwk *call_import_private_jwk(CKeyManager *manager, const CJwk *private_jwk);
CSigner *call_get_signer(CKeyManager *manager, const CJwk *public_jwk);
void poc_key_manager_from_foreign(const CKeyManager *manager);

CKeyManager *new_in_memory_key_manager();
/** --- */

#endif // WEB5_C_H