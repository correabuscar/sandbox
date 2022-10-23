#include <iostream>
using namespace std;

static const void *c=&c; //src: https://cs.chromium.org/chromium/src/chrome/browser/ssl/bad_clock_blocking_page.cc?l=56&rcl=16647ef7b39cf81f20a1589d484799f315da73d3

int main() {
  cout << c<<"\n";
	return 0;
}
