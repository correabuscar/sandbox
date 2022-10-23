
// filename : malloc.h
// To replace all calls to malloc with mymalloc
#define malloc(size) mymalloc(size)
void *mymalloc(size_t size);
