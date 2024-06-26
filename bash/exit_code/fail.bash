#!/usr/bin/env -S PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/bin:${PATH}" bash

set -euf -o pipefail #like the original: https://github.com/pasadoorian/displaygoat/blob/1883636decae12e8a90da5344798a096c673427c/displaygoat.sh#L19
OUTPUT="/tmp/some${RANDOM}.log"
SHOW="dummy"

#re https://github.com/pasadoorian/displaygoat/issues/3
#
set -x
#false || : #use 'false' to assume something returned bad exit code (aka 1) // "# Its okay if this fails, so we append the "always true" e.g. !! :

#ret=0
(if ! /bin/false; then ret=$?; fi) #yeah ofc. this won't work

if [ "$ret" -ne 0 ]; then
  #ret="$?" #can't get that exit code here, so this will be '0' due to the 'if .. then' itself!
  ret="${PIPESTATUS[@]}" #but can still get it via PIPESTATUS array for this `if ! command; then` case!
  echo "feh command failed, this is a big deal" | tee $OUTPUT
  exit $ret  #note though, that in the original he doesn't want to exit because "it's no big deal" ?) we exit anyway I guess.
else
  echo "Success: displaying the images for $SHOW" | tee $OUTPUT
fi
