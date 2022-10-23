//src: https://www.opensourceforu.com/2011/08/lets-hook-a-library-function/
#include<stdio.h>
#include<dlfcn.h>
#include<stdlib.h>

void file1func(int *i);
void file2func(int *i);
int main(void)
{
	void *handler;
	int (*fn) (int *);
	int x;
	char *error;
	handler = dlopen("./libfile.so", RTLD_LAZY);
	if (!handler)
	{
		fprintf(stderr,"%s\n", dlerror());
		exit(1);
	}
	fn = dlsym(handler,"file1func");     /* getting the handle of file1func through dlsym() */
	if ((error = dlerror()) != NULL) /* checking error through dlerror() */
	{
		fprintf(stderr,"%s\n", error);
		exit(1);
	}
	(*fn)(&x);                            /* Calling file1func() to resolve x */
	printf("The value of x is %d\n", x);
	dlclose(handler);                 /* closing the file handle */
	return 0;
}
