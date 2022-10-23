// run like this: --rm ./a.out ; gcc -ggdb3 -O0 mmap_access_beyond.c && ./a.out >screen.out ; ls -la ./screen.out ; cat /tmp/_diff_mmap | colordiff


// https://bugzilla.kernel.org/show_bug.cgi?id=203537

#include <unistd.h> // for close() or sysconf()/_SC_PAGE_SIZE
#include <stdio.h> // for printf()

#define __USE_MISC 1 //to get MAP_FILE or MAP_ANONYMOUS
#include <sys/mman.h> // for mmap()

#include <stdlib.h> //for system()

#include <sys/wait.h> //for wait()

#define SMALL_MMAP 0 //set to 0 to use a 200MiB mmap or set to 1 to use a 1 byte mmap!

int main() {
  off_t size=
#if SMALL_MMAP==1
    1
    // a 1 byte mmap
#else
    200*1024*1024
    // a 200MiB mmap
#endif
  ;
  char *addr;
  int selfpid=getpid();

  int wstatus;
  char *cmd=NULL;
  const unsigned int cmd_size=100;
  cmd=malloc(cmd_size+1);
  snprintf(cmd, 1+cmd_size, "cat /proc/%d/maps >/tmp/_before_mmap", selfpid);
  int rv=system(cmd);
  wait(&wstatus);

  addr = mmap (NULL, size, PROT_READ, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0); //same behaviour even without a file!
  if (addr != MAP_FAILED){
    snprintf(cmd, 1+cmd_size, "cat /proc/%d/maps >/tmp/_after_mmap", selfpid);
    int rv2=system(cmd);
    wait(&wstatus);
    int rv3=system("diff -up /tmp/_before_mmap /tmp/_after_mmap >/tmp/_diff_mmap 2>&1");
    wait(&wstatus);
    // /proc/self/maps idea from `valdis` on ##kernel freenode irc
    // on glibc 2.29 libc-2.29.so follows right after the above mmap, but it's read only! Still, it should SIGBUS as per `man 2/3p mmap`
    //fprintf(stderr,"!! colordiff rv= %d\n",rv3);
    fprintf(stderr,"!! mmap ok %p\n",addr);
//    fprintf(stderr,"!! 1st byte of mmap: %c\n", addr[0]);// SIGBUS error here!
//    fprintf(stderr,"!! 2nd byte of mmap: %c\n", addr[1]);
//    fprintf(stderr,"!! 3nd byte of mmap: %c\n", addr[2]); //works even if size=1
//    const unsigned int page_size=sysconf(_SC_PAGE_SIZE);//4096
//    fprintf(stderr,"!! 0x0FFF-th(PAGE_SIZE-1) byte of mmap: %c\n", addr[page_size-1]); //works even if size=1 
//    fprintf(stderr,"!! 0x1000-th(PAGE_SIZE) byte of mmap: %c\n", addr[page_size]); //works even if size=1 
//    fprintf(stderr,"!! 0xFFFF-th byte of mmap: %c\n", addr[0xFFFF]); //works even if size=1
//    fprintf(stderr,"!! 0xFFFF-th byte of mmap: %c\n", addr[0xFFFF]); //works even if size=1
//    fprintf(stderr,"!! 0X2CFFF-th byte of mmap: %c\n", addr[0x2CFFF]); //works even if size=1
//    fprintf(stderr,"!! 0x2DFFF-th byte of mmap: %c\n", addr[0x2DFFF]); // works even if size=1
//    fprintf(stderr,"!! PAGE_SIZE:%u\n", page_size); // 4096
//    //fprintf(stderr,"!! number of non-zero bytes beyond the end of mmaped-file: ");

    unsigned int nonzerochars_seen=0;
    for (unsigned int i=1; i < size+1551160+0x2E000*2; i++) {
      if ( (i >= 211529728) || ((size == 1) && (i >= 188416)) ) {
        fprintf(stderr,"!! about to access addr at offset i=%u nonzerochars_seen='%u'\n", i, nonzerochars_seen);
      }
      if (addr[i] != 0) {
        nonzerochars_seen++;
      }
      if (nonzerochars_seen>0) { //print all after first non-zero which would be 'ELF'
        printf("%c", addr[i]);
        //XXX ^ crashes at i=211529728 when size == 200MiB, that's on accessing 443rd kernel page after the end of mmap-ed memory region
        //XXX ^ crashes at i = 188416 if size == 1 and screen.out has first 172,016 bytes identical with /lib64/ld-2.29.so which is indirectly listed in a.out's ldd(differently named symlinks to it)
      }
    }
    //fprintf(stderr,"%u\n", nonzerochars_seen);
    fprintf(stderr,"!! 0x2E000-th byte of mmap: %c\n", addr[size+0x2E000-1]); // segfault
    munmap(addr, size);
  }
  return 0;
}
