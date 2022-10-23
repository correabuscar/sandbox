//spawned from/for issue: https://github.com/libcheck/check/issues/188
#include <stdio.h>

#include <sys/stat.h>
#include <fcntl.h>
//#include <bits/fcntl2.h>

#include <unistd.h> //fsync()
#include <errno.h>

#include <sys/file.h> //flock()

#define USE_FOPEN 0 //set to 0 to use open(); set to 1 for fopen() which doesn't have bug!
//so, open() is bugged in local/glibc 2.29.9000.r248.gf6efec90c8-1 (builtbydaddy base)
//local/linux-stable 5.1.2.r0.geb5d65a82f5c-1 (builtbydaddy)
//nope, forgot to pass `mode` arg!!


int main() {
  FILE *f=NULL;
  char *fn="/tmp/broken_perms.log";
#if USE_FOPEN==1
  f = fopen(fn, "a");
  if (NULL == f) {
    fprintf(stderr,"oopsie %d\n", errno);//EINVAL==22
  } else {
    fprintf(stderr, "All good\n");
    fclose(f); // "The file descriptor is not dup'ed, and will  be  closed  when the stream created by fdopen() is closed."
  }
#else
  //const mode_t mode= S_IWUSR | S_IRUSR | S_IRGRP | S_IROTH;
//  char *sigseg=NULL;
//  sigseg[0]=1;
  int fd=open(
     fn,
      O_WRONLY
      //O_RDWR //same effect
      | O_CREAT
      //, mode //FIXME: restore this!
      //, 07777
      ); //XXX FIXED: FORGOT to pass a mode arg which is required for O_CREAT or O_TMPFILE! see `man 2 open`
  if (fd != -1) {
    fprintf(stderr, "All good\n");
  } else {
    perror("open");
  }
  close(fd); // not needed when using fclose(f) above!
#endif
}
