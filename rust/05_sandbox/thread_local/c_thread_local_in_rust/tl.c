// example.c
//#include <stdio.h>

// Define a thread-local variable
__thread int thread_local_var;

// Function to access the thread-local variable
int *get_thread_local_var() {
    return &thread_local_var;
}

// Function to set the value of the thread-local variable
void set_thread_local_var(int value) {
    thread_local_var = value;
}

/*
   In C, thread-local storage (TLS) can be implemented in different ways, and whether it involves heap allocation or new allocations depends on the specific implementation and the platform.

However, the most common and efficient way to implement thread-local storage in C is by using platform-specific mechanisms such as compiler extensions (`__thread` specifier) or standard library functions (`pthread_key_create`, `TlsAlloc`, etc.), which typically allocate thread-local storage directly from the system or using thread-specific data structures managed by the runtime environment.

For example, in the case of the `__thread` specifier in GCC/Clang, thread-local variables are often allocated directly within the thread's control block or using thread-specific registers, avoiding heap allocation entirely.

Similarly, with `pthread_key_create` or Windows-specific functions like `TlsAlloc`, the thread-local storage is managed by the operating system or the runtime environment, typically without involving heap allocation.

That said, it's crucial to understand that the exact implementation details can vary across platforms, compilers, and runtime environments. Therefore, while thread-local storage in C is often implemented efficiently without heap allocation, it's essential to consult the documentation and consider the specific context and requirements of your application to ensure optimal performance and behavior.

*/
