#!/usr/bin/env -S PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/bin:${PATH}" sh

#freebsd's 'sh'

set -vex

[ -z ">" -a -z "a" ] && echo lol
#bash: [: syntax error: `-z' unexpected
[ -z "\>" -a -z "a" ] && echo works
