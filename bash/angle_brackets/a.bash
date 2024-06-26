#!/usr/bin/env -S PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/bin:${PATH}" bash

set -vex

[ -z ">" -a -z "something" ] && echo lol
#bash: [: syntax error: `-z' unexpected
[ -z "\>" -a -z "something" ] && echo works

var=">"
[ -z "$var" -a -z "something" ] && echo lol
#bash: [: syntax error: `-z' unexpected

[ -n "$var" -a -z "something" ] && echo lol
#bash: [: syntax error: `-z' unexpected

[ "$var" = ">" -a -z "something" ] && echo works


#the 'echo hmm' will execute, but it shouldn't
var=">"
[ -n "$var" -a -n "something" ] || echo hmm

#fixed, correctly doesn't execute
var="\>"
[ -n "$var" -a -n "something" ] || echo hmm

#executes, as expected
var=""
[ -n "$var" -a -n "something" ] || echo hmm
