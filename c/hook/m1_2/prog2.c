//src: https://www.opensourceforu.com/2011/08/lets-hook-a-library-function/
#define _GNU_SOURCE
#include <stdio.h>
#include <stdint.h>
#include <dlfcn.h>                               /* header required for dlsym() */
#include <stdbool.h>


// backtrace src: https://www.gnu.org/software/libc/manual/html_node/Backtraces.html
// compile with: gcc -D_FORTIFY_SOURCE=2 backtrace.c -ggdb -O1 -rdynamic
#include <execinfo.h>
//#include <stdio.h>
#include <stdlib.h>

//extern void *__libc_dlsym (void *, const char *);
/* lcheck() is for memory leak check; its code is not shown
 here */
//void lcheck(void);
void print_trace(void);
void* malloc(size_t size)
{
	static void* (*my_malloc)(size_t) = NULL;
  static bool already_in=false;
	if (!my_malloc) {
    already_in=true;
    my_malloc = dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
    //my_malloc = __libc_dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
    already_in=false;
    if (NULL==my_malloc) {
      already_in=true;
      printf("!! EPIC fail...\n");
      already_in=false;
      return NULL;
    }
    //my_malloc = (void * (*)(size_t))dlsym(RTLD_NEXT, "malloc");  /* returns the object reference for malloc */
  }
  if (!already_in) {
    already_in=true;
    printf("!! inside shared object...\n");
    fflush(stdout);
    already_in=false;
  }
	void *p = my_malloc(size);               /* call malloc() using function pointer my_malloc */  
  if (!already_in) {
    already_in=true;
    printf("!! malloc(%ld) = %p\n", size, p);
    fflush(stdout);
    print_trace();
    //lcheck();                                /* calling do_your_stuff function */
    printf("!! returning from shared object...\n");
    fflush(stdout);
    already_in=false;
  }
  //already_in=false; // FIXME: stack trace recursion was only needed because this was uncommented! so remove that code
	return p;
}
//void lcheck(void)
//{
//	printf("!! displaying memory leaks...\n");
//  fflush(stdout);
//	/* do required stuff here */
//}


/* Obtain a backtrace and print it to stdout. */
void
print_trace (void)
{
#define MAX_LINES_IN_STACKDUMP 100
  static bool intrace=false;
  void *array[MAX_LINES_IN_STACKDUMP];
  size_t size;
  char **strings;
  size_t i;

  if (!intrace) {
    intrace=true;

    size = backtrace (array, MAX_LINES_IN_STACKDUMP);
    strings = backtrace_symbols (array, size);

    printf ("!! Obtained %zd stack frames.\n", size);
    fflush(stdout);

    for (i = 0; i < size; i++)
      printf ("!! %s\n", strings[i]);

    fflush(stdout);
    free (strings);
    intrace=false;
  } else {
    printf ("!! stacktrace recursion prevented.\n");
    fflush(stdout);
  }
}
