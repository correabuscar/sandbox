//src: 2011 https://www.opensourceforu.com/2011/08/lets-hook-a-library-function/
#include<stdio.h>
#include<malloc.h>
#include<stdlib.h>
int main(void)
{
	int *p;
	printf("calling from main...\n");
	p=(int *)malloc(10);
	if(!p)
	{
		printf("Got allocation error...\n");
		exit(1);
	}
	printf("returning to main...\n");
	free(p);                           /* freeing memory from heap */
	printf("freeing memory...\n");
	return 0;
}
