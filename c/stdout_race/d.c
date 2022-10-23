//spawned from/for issue: https://github.com/libcheck/check/issues/188
#include <stdio.h>

#include <sys/stat.h>
#include <fcntl.h>

#include <unistd.h> //fsync()
#include <errno.h>

#include <sys/file.h> //flock()

#define USE_FOPEN 0 //set to 0 to use open(); 1 to use fopen; 2 to use open+fdopen!
#define USE_FLOCK 3 //set to 0 to use fcntl(), to 1 to use flock(), to 2 to use no locking!
#define USE_FLF 1// 1 to use flockfile/funlockfile before&after each write! 0 to not
//doneTODO: thanks to <oiaohm> on ##linux (freenode irc): try `man flockfile` and see that SO question: https://stackoverflow.com/questions/467938/stdout-thread-safe-in-c-on-linux

#include <sys/sysmacros.h> // for minor() major()


//doneTODO: <twkm> and <Learath2> suggested to check stat/fstat
// <twkm> [...] compare certain stat fields
// <Learath2> howaboutsynergy: btw what you are looking for is st_ino and st_dev, in conjunction with fstat, on a POSIX conforming system 
// Learath2> st_ino and st_dev pair identifies a file uniquely afaik
//
void statme(int fd) {
    struct stat sb;
    if (fstat(fd,&sb) == -1)  {
      perror("fstat");
    } else {
#if USE_FLF==1
      flockfile(stderr);
#endif
      fprintf(stderr,"stat for filedes %d follows:\n", fd);
#if USE_FLF==1
      funlockfile(stderr);
      flockfile(stderr);
#endif
      fprintf(stderr, "ID of containing device:  [%lx,%lx]\n",
                (long) major(sb.st_dev), (long) minor(sb.st_dev));
#if USE_FLF==1
      funlockfile(stderr);
      flockfile(stderr);
#endif
      fprintf(stderr,"I-node number:            %ld\n", (long) sb.st_ino);
#if USE_FLF==1
      funlockfile(stderr);
#endif
    }
}

int main() {
  FILE *f=NULL;
#if USE_FOPEN==1
  //f = fopen("/tmp/a_out_.log", "aw"); //XXX: bad, it's append only, apparently! https://gist.github.com/howaboutsynergy/4dc0c41d6244d91a7dfd07159b905fe9#gistcomment-2919552
  //f = fopen("/tmp/a_out_.log", "wa"); //good, it creates file every time! but undocumented and probably ignores 'a' - don't use!
  f = fopen("/tmp/a_out_.log", "w"); //TODO: try freopen() with "a" after this!
#elif USE_FOPEN==0 || USE_FOPEN==2
  //int fd=open("/tmp/a_out_.log", O_APPEND | O_CREAT | O_SYNC /*| O_EXCL*/ ); //doesn't work with fdopen(,"a")!
  //int fd=open("/tmp/a_out_.log", O_APPEND | O_CREAT | O_SYNC | O_EXCL ); //doesn't work with fdopen(,"a")!
  //int fd=open("/tmp/a_out_.log", O_CREAT | O_WRONLY);
  int fd=open("/tmp/a_out_.log", O_APPEND | O_WRONLY | O_CREAT | O_TRUNC /*|O_SYNC /*| O_EXCL*/ 
       , S_IWUSR | S_IRUSR | S_IRGRP | S_IROTH
      ); //doesn't work with fdopen(,"a")!
  //int fd=open("/tmp/a_out_.log", O_WRONLY | O_CREAT | O_SYNC | O_EXCL );//works
  if (fd > -1) {
    statme(fd);
    statme(1);//fd 1 is stdout (0=stdin; 2=stderr)
    statme(2);//fd 1 is stdout (0=stdin; 2=stderr)
#if USE_FLOCK==0
    struct flock fl = {F_WRLCK, SEEK_SET, 0, 0, 0}; //src: https://stackoverflow.com/questions/13159964/file-locking-compatible-with-fgets-and-fprintf?r=SearchResults
    if (fcntl(fd, F_SETLK, &fl) == -1) {
        perror("fcntl-setlock");
    }
    //XXX lock works but has no effect! Actually it should fail, but probably doesn't because it's a new fd ? I don't get it!
#elif USE_FLOCK==1
    //<ayecee> howaboutsynergy: flock is advisory lock. you'd need a mandatory lock. i don't know how to do that offhand.
    //TODO: <ayecee> howaboutsynergy: maybe lockf?
    //XXX: On Linux, lockf() is just an interface on top of fcntl(2) locking.
    if (flock(fd, LOCK_EX)) {
      perror("flock-set");
    }
#endif
#if USE_FOPEN==2
    f = fdopen(fd, "a");
#else
    f = fdopen(fd, "w"); // Modes "w" or "w+" do not cause truncation of the file.
#endif
    //f = fopen("/tmp/a_out_.log", "a");
#endif
    if (NULL == f) {
      fprintf(stderr,"oopsie %d\n", errno);//EINVAL==22
    } else {
#if USE_FLF==1
      flockfile(stdout);
#endif
      fprintf(stdout, "Something");
#if USE_FLF==1
      funlockfile(stdout);
#endif

      //fsync(stdout);
      fsync(1);
      sync();
      fprintf(f," messy ");
#if USE_FOPEN!=1
      fsync(fd);
#endif
      sync();
#if USE_FLF==1
      flockfile(f);
#endif
      fprintf(f," jessy\n");
#if USE_FLF==1
      funlockfile(f);
#endif
#if USE_FOPEN!=1
      fsync(fd);
#endif
      sync();
#if USE_FLF==1
      flockfile(stdout);
#endif
      fprintf(stdout, " or another\n");
#if USE_FLF==1
      funlockfile(stdout);
#endif
      //fsync(stdout);
      fsync(1);
      sync();
#if USE_FLF==1
      flockfile(f);
#endif
      fprintf(f,"More stuff\n");
#if USE_FLF==1
      funlockfile(f);
#endif
#if USE_FOPEN!=1
      fsync(fd);
#endif
      sync();
#if USE_FLOCK==0
      fl.l_type = F_UNLCK;
      if (fcntl(fd, F_SETLK, &fl) == -1) { //has to be before fclose(f)
        perror("fcntl-close");
      }
#elif USE_FLOCK==1
      if (flock(fd, LOCK_UN)) {
        perror("flock-unlock");
      }
#endif
      fclose(f);
      sync();
    }
#if USE_FOPEN!=1
  } else {
    perror("open");
    //fprintf(stderr, "no open\n");
  }
  close(fd);
#endif
  sync();
}
