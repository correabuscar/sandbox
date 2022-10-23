#include <fcntl.h> // open, O_CREAT
#include <unistd.h> // close
#include <stdio.h> // perror


int main() {
  int fd=open("/tmp/temp.file.tmp", O_CREAT); // clang doesn't catch this
  if (fd != -1) {
    perror("open");
  } else {
    close(fd);
  }
  int fd2=open("/tmp/temp.file.tmp", O_RDWR, S_IWUSR | S_IRUSR, 1,2,3,4); // caught by `clang-analyzer-unix.API`
}
