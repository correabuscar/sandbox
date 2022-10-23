// https://bugzilla.kernel.org/show_bug.cgi?id=203537
// https://midnight-commander.org/ticket/3983#comment:7
#include <sys/types.h>
#include <sys/stat.h>

#define __USE_XOPEN2K8 1 //to get O_NOFOLLOW
#include <fcntl.h>

#include <unistd.h> // close
#include <stdio.h> //printf

#define __USE_MISC 1 //to get MAP_FILE
#include <sys/mman.h> //mmap

#include <stdlib.h>
#include <bits/mman-linux.h> // MAP_FILE
#include <string.h> // memcmp

#define CRASH 0
//^ set to 0 or undefine it to not crash!
#define USE_SYMLINK 0
#define USE_BIGFILE 1 //set to 0 to use 1 byte file when USE_SYMLINK is 0 (and CRASH is 0)

int main() {
  int file;
  off_t size=1; //should be size of the contents of the file that the symlink points to!
#if CRASH==1
  file = open("./3/symlink_to_emptyfile", O_RDONLY // | O_NOFOLLOW //open will fail since the file is a symlink! 12 bytes symlink (ie. "../emptyfile")
      );
#else //don't crash:
#if USE_SYMLINK==1
  fprintf(stderr,"!! symlink\n");
  file = open("./3/symlink_to_1bytefile", O_RDONLY); //works!
#elif USE_BIGFILE==0
  fprintf(stderr, "!! normal file\n");
  file = open("./1bytefile", O_RDONLY); //works!
#else
  fprintf(stderr,"!! big file\n");
  file = open("./bigfile", O_RDONLY); // created via: dd if=/dev/zero of=bifgile bs=1M count=200
  size=200*1024*1024;
#endif
#endif
  if (file >= 0) {
    fprintf(stderr,"!! open success\n");
    char *addr;
    //addr = mmap (NULL, size, PROT_READ, MAP_FILE | MAP_PRIVATE, file, 0);
    addr = mmap (NULL, size, PROT_READ, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0); //same behaviour even without a file!
    if (addr != MAP_FAILED){
      fprintf(stderr,"!! mmap ok %p\n",addr);
      fprintf(stderr,"!! 1st byte of mmap: %c\n", addr[0]);// SIGBUS error here!
      fprintf(stderr,"!! 2nd byte of mmap: %c\n", addr[1]);
      fprintf(stderr,"!! 3nd byte of mmap: %c\n", addr[2]); //works even if size=1
      const unsigned int page_size=sysconf(_SC_PAGE_SIZE);//4096
      fprintf(stderr,"!! 0x0FFF-th(PAGE_SIZE-1) byte of mmap: %c\n", addr[page_size-1]); //works even if size=1 
      fprintf(stderr,"!! 0x1000-th(PAGE_SIZE) byte of mmap: %c\n", addr[page_size]); //works even if size=1 
      fprintf(stderr,"!! 0xFFFF-th byte of mmap: %c\n", addr[0xFFFF]); //works even if size=1
      fprintf(stderr,"!! 0xFFFF-th byte of mmap: %c\n", addr[0xFFFF]); //works even if size=1
      fprintf(stderr,"!! 0X2CFFF-th byte of mmap: %c\n", addr[0x2CFFF]); //works even if size=1
      fprintf(stderr,"!! 0x2DFFF-th byte of mmap: %c\n", addr[0x2DFFF]); // works even if size=1
      fprintf(stderr,"!! PAGE_SIZE:%u\n", page_size); // 4096
      fprintf(stderr,"!! number of non-zero bytes beyond the end of mmaped-file: ");
      unsigned int count=0;
      for (unsigned int i=1; i < size+1551160+0x2E000*2; i++) {
        if (addr[i] != 0) {
          count++;
        }
        if (count>0) { //print all after first non-zero which would be 'ELF'
          printf("%c", addr[i]); //XXX crashes at i=211529728, that's on accessing 443rd kernel page after the end of file
        }
/*        if ((count > 0) && (count % 10000 == 0)) {
          printf("!! i=%u count='%u'\n", i, count);
        }*/
      }
      fprintf(stderr,"%u\n", count);
      fprintf(stderr,"!! 0x2E000-th byte of mmap: %c\n", addr[size+0x2E000-1]); // segfault
      munmap(addr, size);
    }
  } else {
    fprintf(stderr,"!! open failed\n");
  }
  close(file);
  return 0;
}
