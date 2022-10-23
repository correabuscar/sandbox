#define OFFICIAL_BUILD 1

int main() {
//#if !defined(OFFICIAL_BUILD) && !defined(OS_NACL) && !defined(__UCLIBC__) && \
//    !defined(OS_AIX) || true //so it works with || true here!
#if false && false && false || true
  return 1;
#endif
}
