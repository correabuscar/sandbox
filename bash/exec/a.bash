#!/bin/bash

set -x

#export environ="" #not what i thought

echo "\$0 = '$0' \$*='$*'"
#set
#what you get:
#$0 = './a.bash'
#$0 = '/${HOME}/sandbox/bash/exec/a.bash'  <- not "custom" !!


cleanup() {
trap - EXIT
  exec -l -a "custom" -- "$0" "$@"
}

#trap cleanup EXIT
if test "$1" == "s"; then
  exec -l -a "custom" -- "$0" e "$@"
  #exec "$0" #same thing!
fi

sleep 20
