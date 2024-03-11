#!/usr/bin/env bash

#set -euf -o pipefail #like the original: https://github.com/pasadoorian/displaygoat/blob/1883636decae12e8a90da5344798a096c673427c/displaygoat.sh#L19
OUTPUT="/tmp/some${RANDOM}.log"
SHOW="dummy"

#re https://github.com/pasadoorian/displaygoat/issues/3
#
#false || : #assume something returned bad exit code (aka 1) // "# Its okay if this fails, so we append the "always true" e.g. !! :

set -x
false
ret=$?
if [ "$ret" -ne 0 ]; then
  echo "feh command failed, this is a big deal" | tee $OUTPUT
  exit $ret
else
  echo "Success: displaying the images for $SHOW" | tee $OUTPUT
fi
