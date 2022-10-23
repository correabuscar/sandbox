//src: https://stackoverflow.com/q/6653603/11509478
#include <stdio.h>
#include <stdlib.h>

int main(){
    char c;
    setvbuf(stdout, NULL, _IOLBF, BUFSIZ);
    printf("Hello world\n");
    c = getchar();
    printf("got char: %c\n", c);
}
