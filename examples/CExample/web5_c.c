#include <stdio.h>
#include <stdlib.h>
#include "web5_c.h"

int main() {
    const char* did_uri = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";
    const char* gateway_url = NULL;  // Using the default gateway

    char* result = did_dht_resolve(did_uri, gateway_url);

    if (result != NULL) {
        printf("%s", result);
        free_string(result);
    } else {
        printf("Failed to resolve DID\n");
    }

    return 0;
}