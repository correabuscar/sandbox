#include <stdio.h>
#include <stdlib.h>

// Declare secure_getenv if not available (not standard)
#ifndef _GNU_SOURCE
extern char *secure_getenv(const char *);
#endif

int main() {
    const char* debug_var = secure_getenv("DEBUG");

    if (debug_var != NULL) {
        printf("DEBUG environment variable (via secure_getenv): %s\n", debug_var);
    } else {
        printf("DEBUG environment variable not set or inaccessible via secure_getenv.\n");
    }

    // Now, let's retrieve the variable using getenv() to compare
    debug_var = getenv("DEBUG");

    if (debug_var != NULL) {
        printf("DEBUG environment variable (via getenv): %s\n", debug_var);
    } else {
        printf("DEBUG environment variable not set or inaccessible via getenv.\n");
    }

    return 0;
}

