
#include <stdio.h>
int main() {
  int a=16;
  {
    int a=4;
    printf("a in  =%d\n",a);
  }
  printf("a out =%d\n",a);
}
