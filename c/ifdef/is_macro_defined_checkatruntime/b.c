#define STRINGIFY_ITS_VALUE(x) #x

#define STRINGIFY_HELPER(x) #x
#define STRINGIFY(x) STRINGIFY_HELPER(x)
//#define PREFIX "PREFIX_"
//#define STRINGIFY_HELPER(x) PREFIX #x
//#define STRINGIFY(x) STRINGIFY_HELPER(PREFIX ## x)
//#define STRINGIFY2(x) STRINGIFY(x)

#define EXAMPLE 1
//#define MY_MACRO(name) (STRINGIFY(name))

//XXX: can't detect this, otherwise it would work to detect if any MACRO_NAME is or isn't defined
#define SAMESELF SAMESELF

#include <stdio.h>
#include <string.h> // Include <string.h> for strncmp

//#define AMY_MACRO(name) (defined(name) ? #name : "")
//#define AMY_MACRO(name) (STRINGIFY(name[0]) != '\0' && defined(name) ? STRINGIFY(name) : "")
//#define IS_DEFINED(x) (STRINGIFY(x)[0] != '\0')
#define IS_DEFINED(x) \
  do { \
    size_t len1=sizeof(STRINGIFY_ITS_VALUE(x)); \
    size_t len2=sizeof(#x); \
    size_t maxlen= len1>len2?len1:len2; \
    if (strncmp(STRINGIFY_ITS_VALUE(x), #x, maxlen) != 0) { \
      printf("defined "#x"\n"); \
    } else {\
      printf("NOT defined "#x"\n"); \
    } \
  } while(0)

//if (strcmp(STRINGIFY_ITS_VALUE(x), #x) != 0) { 
//if (STRINGIFY_ITS_VALUE(x) != #x) {


int main() {
//    const char *result = MY_MACRO(EXAMPLE); // expands to "1"
//    printf("%s\n", result);
//    printf("%d\n",IS_DEFINED(FOO));
//    printf("%d\n",IS_DEFINED(EXAMPLE));
//    printf("%s\n",STRINGIFY(EXAMPLE));
//    printf("%s\n",STRINGIFY(EXAMPLE2));
//    printf("!%s!\n",STRINGIFY2(UNDEFINED1));
    printf("!%s!\n",STRINGIFY(UNDEFINED1));
    printf("!%s!\n",STRINGIFY_HELPER(UNDEFINED1));
//    printf("!%s!\n",STRINGIFY2(SAMESELF));
    printf("!%s!\n",STRINGIFY(SAMESELF));
    printf("!%s!\n",STRINGIFY_HELPER(SAMESELF));
    IS_DEFINED(RANDOMUNDEFINEDONE); //wrongly reports as undefined
    IS_DEFINED(SAMESELF); //wrongly reports as undefined
    IS_DEFINED(EXAMPLE); // correctly reports defined
    return 0;
}

