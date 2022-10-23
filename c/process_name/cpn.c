//for parsing /proc/self/cmdline, src: https://stackoverflow.com/questions/24127416/parsing-command-line-arguments-from-proc-pid-cmdline/24128544#24128544

//for close()
#include <unistd.h>
//for open()
#include <fcntl.h>
//for printf:
#include <stdio.h>

//for realpath()
#include <limits.h> /* PATH_MAX */
#include <stdlib.h>


//#include "string.h" //strerror
//#include "errno.h" //errno

#include <unistd.h> //page size

int main() {
  char buf[PATH_MAX]; /* PATH_MAX incudes the \0 so +1 is not required */ //src: https://stackoverflow.com/questions/1563168/example-of-realpath-function-in-c/1563237#1563237
  char *rp=realpath("/proc/self/exe",buf);
  //char *rp=realpath("/proc/self/exe",NULL);
  if (NULL != rp) {
    printf("The program '%s' was called as:\n",rp);
  }else{
    //char* errStr = strerror(errno);
    //printf("error string: %s\n", errStr);
    perror("realpath");
  }

  const long PAGESIZE = sysconf(_SC_PAGESIZE); //ie. 4096, run at prompt: $ getconf PAGESIZE  (getconf is part of sys-libs/glibc, on Gentoo)
	const long BUFSIZE = PAGESIZE; // should really get PAGESIZE or something instead...
	unsigned char buffer[BUFSIZE]; // dynamic allocation rather than stack/global would be better
	int fd = open("/proc/self/cmdline", O_RDONLY);
	int nbytesread = read(fd, buffer, BUFSIZE);
	unsigned char *end = buffer + nbytesread;
	for (unsigned char *p = buffer; p < end; /**/)
	{
		printf("'%s' ",p);
		while (*p++); // skip until start of next 0-terminated section
	}
  printf("\b\n");//erase last space char and add new line!
  fflush(stdout);
	close(fd);
}
