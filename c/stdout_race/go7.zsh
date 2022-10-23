#!/bin/zsh

set -e
rm ./a.out || true
gcc -D_FORTIFY_SOURCE=2 -O2 g.c
set +e
./a.out > /tmp/go7.log
echo 'first:'
cat /tmp/go7.log
echo ./a.out | sh > /tmp/go7_2.log  #src: https://stackoverflow.com/a/4050395/11509478
echo 'second:'
cat /tmp/go7_2.log
