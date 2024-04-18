#include <stdio.h>

#define STRINGIFY_HELPER(x) #x
#define STRINGIFY(x) STRINGIFY_HELPER(x)
#define STRINGIFY2(x) STRINGIFY(x)

#define EOLN "\n"

#define EXPECT_MISS(type, id) \
  printf("Missing def(expected): " #id EOLN);

#define UNEXPECTED_MISS(type, id) \
  printf("Unexpected missing def: " #id EOLN);

#define PCONST(type, id) \
  printf("Defining def: " #type " " #id EOLN);

#define MYMISS(type, id) \
  MY(type,id, EXPECT_MISS)

#define MYUEM(type,id) \
  MY(type,id, UNEXPECTED_MISS)

#define MY(type, id, CALL_WHEN_EMPTY) \
  do { \
    if (""STRINGIFY(id)[0] == '\0') { \
            CALL_WHEN_EMPTY(type, id); \
        } else { \
            PCONST(type, id); \
        } \
    } while (0)


int main() {
  MYMISS(i32, KEY_EVENT);
  MYUEM(i32, KEY_SHOULDEXIST);

  return 0;
}
