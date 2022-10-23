//XXX nevermind, it was actually Show File from chrome://net-export/ that made the file a+rwx
#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <unistd.h> //close and sync

int main() {
//  int fd=open("/tmp/a_out_.log",
//      //O_EXCL |
//      O_CREAT |  O_TRUNC
//      | O_WRONLY
//      //| O_RDWR
//   ,  S_IRUSR | S_IWUSR
//   );
  //mode_t mode = S_IRUSR | S_IWUSR;
  umask(0);
  int mode2=0b111111111;//2147483647-1;//4294967296-1;
//  mode_t mode= S_IRWXU | S_IRWXG | S_IRWXO;
  int mode = S_IRUSR | S_IWUSR;

  int open_flags=0;
//  open_flags=
//      //O_EXCL |
//      O_CREAT |  O_TRUNC
//      | O_WRONLY
//      //| O_RDWR
//      ;
  //int fd=open("./a.log",
  int fd=open("/tmp/a.log",
      open_flags
   ,  mode
   );
  if (fd > -1) {
  }
  //close(fd);
  //sync();
  printf("mode =%d\n",mode);
  printf("mode2=%d\n",mode2);
  printf("sizeof(mode)=%d\n",sizeof(mode));
  printf("sizeof(open_flags)=%d\n",sizeof(open_flags));
  chdir("/tmp/chrome-net-export-log2.json");
}
