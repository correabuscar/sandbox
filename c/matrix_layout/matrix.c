#include <stdint.h>
#include <stdio.h>

#define MAX 2
typedef uint8_t sometype;

static sometype const matrix[MAX][MAX] = {
  {1,2},
  {3,4}
};

int main() {
  for (int i=0; i<MAX; i++) {
    printf("%d\n", matrix[i][0]);
  }
  return 0;
}
