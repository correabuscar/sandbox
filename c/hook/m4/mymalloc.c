//src: 2017 https://www.geeksforgeeks.org/function-interposition-in-c-with-an-example-of-user-defined-malloc/
// filename : mymalloc.c
/* Link-time interposition of malloc using the
   static linker’s (ld) "--wrap symbol" flag. */
#include <stdio.h>
  
// __real_malloc() is used to called actual library
// malloc()
void *__real_malloc(size_t size);
  
// User defined wrapper for malloc()
void *__wrap_malloc(size_t size)
{
   printf("My malloc called\n");
   return NULL;
}
