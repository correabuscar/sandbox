//obsoleted, now continued as ../interleaving_output/*

//src: https://stackoverflow.com/q/8960611/11509478
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> //for getpid()
#include <sys/time.h> //for gettimeofday() and timeval

#define EXTRADELAY 0
//^ set to 1 to enable extra delay; to 0 to disable; but this doesn't matter!

#define FLUSHEACHLINE 0
//^ set to 1 to flush after each line; to 0 to never flush

#define HOWMANY 110
//^ 21+HOWMANY is size of fprintf-ed string!

void random_seed(){
    struct timeval tim;
    gettimeofday(&tim, NULL);
    double t1=tim.tv_sec+(tim.tv_usec/1000000.0);
    srand (t1);
}

int rrnd() {
#if EXTRADELAY==1
  struct timespec t;
  t.tv_nsec=100000;
  t.tv_sec=0;
  nanosleep(&t, NULL);
#endif
  return rand()%10;
}

//TODO: see `do_write = to_do - (block_size >= 128 ? to_do % block_size : 0);` in func. _IO_new_file_xsputn() of file libio/fileops.c in glibc source code! ie. /home/user/build/1packages/4used/glibc/makepkg_pacman/glibc/src/glibc/libio/fileops.c
int main(){
    FILE *f;
    int i;
    //int size=127;//_IOFBF & _IOLBF interleave when size < 128 !!! see glibc/libio/fileops.c line: `do_write = to_do - (block_size >= 128 ? to_do % block_size : 0);`
    //int size=128; //all variants below work good(no interleaving) when size is 128! (but only with fflush(f) else _IOFBF will fail!)
    //int size=12;
    //int size=13;
    //int size=49;//so if text to be written(51(not counting \0) chars) is bigger than block_size even if block_size is >=128 (currently set to 13 in libio/fileops.c ) then interleaving still happens! even with _IOLBF AND fflush(f) at the same time!
    //int size=227; //still works, same as above tho
    //int size=BUFSIZ; //still works, same as above
    //int size=139;
    //int size=HOWMANY+21; //bigger than 128 and bigger than minimum buffer to hold the fprintf-ed string. // no interleaving!
    //int size=HOWMANY+21-1; //bigger than 128 and less than min. buffer... => interleaving! (only seeing the \n char here)
    int size=HOWMANY+21-2; //bigger than 128 and less than min. buffer... => interleaving! (more obvious)
    char *buf=(char*)malloc(size);

    f = fopen("/tmp/output.txt", "a");
    //setvbuf (f, buf, _IOFBF, size); // 1. fully buffered: fail! even more granular interleaving than _IOLBF below:
    setvbuf (f, buf, _IOLBF, size); // 2. line buffered: fail! interleaving!(only when size<128)
    //setvbuf (f, buf, _IONBF, size); // 3.a. XXX: unbuffered: good - no interleaving
    //setvbuf (f, NULL, _IONBF, 0); // 3.b. XXX: unbuffered: good - no interleaving
    //3.c. no setvbuf() call? acts like unbuffered when FLUSHEACHLINE=1, or acts like _IOFBF(fully buffered) when FLUSHEACHLINE==0 => XXX: good - no interleaving in the first case, bad: interleaving in second case!
    random_seed();

    char *s=(char *)malloc(HOWMANY+1);
    //int j=0;
    for (int j=0; j<HOWMANY; j++) {
      //s[j]=(char)('a'+j);
      snprintf((char *)s+j, 2, "%d", (j % 10) );//2 so it puts the 0 also, else I get a warning
    }
    //s[j]='\0';//now not needed
    //fprintf(stdout,"!%s!%lu!\n", s, strnlen(s, HOWMANY*2));
    for(i=0; i<2000; i++){
          //fprintf(f, "[ xx - %d - 012345678901234567890123456789 - %d]\n", rrnd() , getpid());
          fprintf(f, "[ xx - %d - %s - %05d]\n", rrnd() , s, getpid());// 11+HOWMANY+3+5+1+1=21+HOWMANY=51
#if FLUSHEACHLINE==1
          fflush(f); // XXX: this is what allowed _IO_FBF to also work when only _IO_LBF worked(without this)!
#endif
//TODO: retest without fflush(f) ! find out why libio/fileops.c doesn't write the end of the block too! and block size is 'size' if size>=128
          //sync(); //no effect
    }

    fclose(f);
    free(buf);
    free(s);
}
