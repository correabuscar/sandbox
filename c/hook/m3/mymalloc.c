/* Compile-time interposition of malloc using C preprocessor. 
   A local malloc.h file defines malloc as wrapper */
  
// A file that contains our own malloc function
// File Name : mymalloc.c
#include <stdio.h>
#include <malloc.h>
void *mymalloc(size_t s)
{
   printf("My malloc called\n");
   return NULL;
}

