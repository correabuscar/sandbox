#include <stdio.h>
#include <netinet/in.h>

int main() {
    printf("Size of struct in_addr: %zu\n", sizeof(struct in_addr));
    return 0;
}

