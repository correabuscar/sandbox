// src: https://www.gnu.org/software/libc/manual/html_node/Backtraces.html
// compile with: gcc -D_FORTIFY_SOURCE=2 backtrace.c -ggdb -O1 -rdynamic
// run with: ./a.out
#include <execinfo.h>
#include <stdio.h>
#include <stdlib.h>

/* Obtain a backtrace and print it to stdout. */
void
print_trace (void)
{
#define MAX_LINES_IN_STACKDUMP 100
  void *array[MAX_LINES_IN_STACKDUMP];
  size_t size;
  char **strings;
  size_t i;

  size = backtrace (array, MAX_LINES_IN_STACKDUMP);
  strings = backtrace_symbols (array, size);

  printf ("Obtained %zd stack frames.\n", size);

  for (i = 0; i < size; i++)
     printf ("%s\n", strings[i]);

  free (strings);
}

/* A dummy function to make the backtrace more interesting. */
void
dummy_function (void)
{
  print_trace ();
//  char *p=NULL;
//  *p=1; //if I don't keep this, then dummy_function is inlined, so won't see it in the dump!
}

int
main (void)
{
  dummy_function ();
  return 0;
}
