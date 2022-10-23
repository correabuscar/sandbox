//spawned from/for issue: https://github.com/libcheck/check/issues/188
#include <stdio.h>

#include <sys/stat.h>
#include <fcntl.h>

#include <unistd.h> //fsync()
#include <errno.h>

#include <sys/file.h> //flock()

#define USE_FOPEN 0 //set to 0 to use open(); set to 1 for fopen() which doesn't have bug!
#define USE_FDOPEN 0 //set to 0 to not!

int main() {
  FILE *f=NULL;
  char *fn="/tmp/broken_perms.log";
#if USE_FOPEN==1
  f = fopen(fn, "a");
#else
  int fd=open(fn, O_WRONLY | O_CREAT);
  if (fd != -1) {
#if USE_FDOPEN==1
    f = fdopen(fd, "w");
#endif
#endif
#if USE_FDOPEN==1
    if (NULL == f) {
      fprintf(stderr,"oopsie %d\n", errno);//EINVAL==22
    } else {
      fclose(f); // "The file descriptor is not dup'ed, and will  be  closed  when the stream created by fdopen() is closed."
    }
#endif
#if USE_FOPEN!=1
  } else {
    perror("open");
  }
#endif
#if USE_FDOPEN!=1
  close(fd); // not needed when using fclose(f) above!
#endif
}
