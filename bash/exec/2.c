#include <stdio.h>
#include <unistd.h>
int main(int argc, char **argv)
{
   printf("%s\n",argv[0]);
   sleep(2);
   return 0;
}
