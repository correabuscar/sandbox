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

int main() {
  int file, file2;
  off_t size=12;
  file = open("./3/symlink_to_emptyfile", O_RDONLY // | O_NOFOLLOW //open will fail since the file is a symlink! 12 bytes symlink
      );
  file2=open("./5/emptyfile", O_RDONLY); //file is a 12 bytes file! not symlink!
  if ((file >= 0)&&(file2 >=0)) {
    printf("!! open success\n");
    char *addr;
    //char *addr2;
    addr = mmap (0, size, PROT_READ, MAP_FILE | MAP_PRIVATE, file, 0);
//    addr2 = mmap (0, size, PROT_READ, MAP_FILE | MAP_PRIVATE, file2, 0);
    if (addr != MAP_FAILED){ // && addr2 != MAP_FAILED) {
      printf("!! mmap ok\n");
      //printf("!! 1byte of file: %c\n", addr2[0]); //works
      printf("!! 1byte of symlink: %c\n", addr[0]);
      //int result = memcmp (addr, addr2, size);
      //printf("!! memcmp result=%d\n", result);
      munmap(addr, size);
      //munmap(addr2, size);
    }
  } else {
    printf("!! open failed\n");
  }
  close(file);
  close(file2);
  return 0;
}
