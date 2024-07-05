#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <limits.h>

int main() {
    char *str = "-21";
    char *endptr;
    //unsigned long result;
    long result;

    // Convert string to unsigned long, but store it in a 'long' lol - seen this in 'git diff' code, but it's later on capped at 0
    // better store it as negative than as 18446744073709551595 i guess :)
    result = strtoul(str, &endptr, 10);

    // Check for conversion errors
    if ((errno == ERANGE && (result == ULONG_MAX))
        || (errno != 0 && result == 0)) {
        perror("strtoul");
        return EXIT_FAILURE;
    }

    // Print the result
    //printf("Converted value: %lu\n", result);
    printf("Converted value: %ld\n", result); // it is -21 aka negative

    return EXIT_SUCCESS;
}

