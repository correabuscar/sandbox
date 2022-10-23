#include <stdio.h>

int main() {
  FILE *f=NULL;
  fprintf(stdout, "First stdout line!\n");
  //f = fopen("/tmp/a_out_.log", "a");  // <-- here is the change
  f = fopen("/tmp/a_out_.log", "w");
  if (NULL == f) {
    fprintf(stderr,"oopsie\n");
  } else {
    fprintf(stdout, "Something");
    fprintf(f," messy ");
    fprintf(stdout, " not");
    fprintf(f," jessy\n");
    fprintf(stdout, " another\n");
    fprintf(f,"More stuff and simple tricks and nicks\n");
    fclose(f);
  }
}
