//src: 2017 https://www.geeksforgeeks.org/function-interposition-in-c-with-an-example-of-user-defined-malloc/
// File Name : hello.c
  
#include <stdio.h>
#include <stdlib.h>
#include <malloc.h>
  
int main(void)
{
    // Call to user defined malloc
    void *ptr = malloc(4);
      
    printf("Hello World\n");
    return 0;
}
