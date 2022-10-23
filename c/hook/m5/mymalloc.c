//src: 2017 https://www.geeksforgeeks.org/function-interposition-in-c-with-an-example-of-user-defined-malloc/

/* Run-time interposition of malloc based on dynamic linker’s
   (ld-linux.so) LD_PRELOAD mechanism */
#define _GNU_SOURCE
#include <stdio.h>
  
void *malloc(size_t s)
{
   printf("My malloc called\n");
   return NULL;
}
