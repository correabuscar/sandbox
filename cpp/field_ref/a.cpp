#include <stdio.h>

int ELSE=30;
class A {
  public:
    int SOME=ELSE+1; //field 'ELSE' is uninitialized when used here [-Wuninitialized]
    int ELSE=2; //it works if I place this first!
};

int main() {
  A *a = new A();
  printf("%d\n", a->SOME); // 1
  delete a;
}
