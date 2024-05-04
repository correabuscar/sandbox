#include <stdio.h>

extern void my_rust_function();

int main() {
    printf("Calling Rust function...\n");
    my_rust_function();
    printf("Back to C program.\n");
    return 0;
}
