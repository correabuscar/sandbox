#!/usr/bin/env sh

#freebsd's 'sh'

set -vex

[ -z ">" -a -z "a" ] && echo lol
#bash: [: syntax error: `-z' unexpected
[ -z "\>" -a -z "a" ] && echo works
