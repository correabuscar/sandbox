//src: https://stackoverflow.com/q/4050261/11509478

#include <stdio.h>
int main()
{
  int a=5;
  //setvbuf (stdin, NULL, _IOLBF, BUFSIZ); //src: https://stackoverflow.com/a/4050308/11509478 XXX: obviously doesn't work!
  //setvbuf (stdout, NULL, _IOLBF, BUFSIZ); //works:
  //setvbuf (stdout, NULL, _IONBF, 0); //works:
  //setvbuf (stdout, NULL, _IOFBF, BUFSIZ); // fully buffered (default - only because it's redirected to file!) - doesn't work!

  //uncomment both(nobuf+FBF) and you also see 'a=': (in both bash and zsh)
  setvbuf (stdout, NULL, _IONBF, 0); setvbuf (stdout, NULL, _IOFBF, BUFSIZ);
  //XXX: both lines are needed!

  //setvbuf (stdout, NULL, _IONBF, 0); setvbuf (stdout, NULL, _IOLBF, BUFSIZ); //same for this LBF! maybe because I've bash patch: always_append_on_redirection.patch ? nope, seems to work the same with zsh(unpatched) see ./go7.zsh and also with upstream archlinux bash package (aka unpatched!)

  printf("Hello World\n");
  printf("a=%s\n", a); //intentional segfault
}
