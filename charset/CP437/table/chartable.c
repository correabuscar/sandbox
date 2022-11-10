#include <stdio.h>

int main() {
  for (int i=0; i<=255; i++) {
    unsigned char c = i & 0xff;
    printf("%c",c);// & 0xff);
    /*if (i%32==0)
      printf("\n");*/
    /*if (255==i)
      break;*/
  }
  //printf("\n");
}
