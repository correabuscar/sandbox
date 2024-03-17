#include <stdio.h>

// Mark all symbols as hidden by default
//#pragma GCC visibility push(hidden)

#include "lib.h"


void deprecated_function() {
    printf("This function is deprecated.\n");
}

// Restore default visibility
//#pragma GCC visibility pop
