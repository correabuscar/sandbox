#!/bin/bash

#trap SIGINT won't return proper exit code if interrupted command was builtin!

cleanup() {
	local ec="$?"
	trap - EXIT SIGINT
  echo "cleaned(ec=$ec) argc=$# argv='$*'"
  #exit "$ec"
  if test "$1" == "sigint"; then
    exit "$ec"
  #else #it's exit, no need to re-exit!
    #true
    #return "$ec"
  fi
}

#trap cleanup EXIT SIGINT
trap 'cleanup sigint' SIGINT
trap 'cleanup exit' EXIT
#xfce4-terminal --disable-server --geometry 80x24 -x ccachewatch
#echo "Press C-c here"
#sleep 100 #press C-c here, XXX why is exit code 130 correctly detected here,
read -rp "Press C-c here..." #but not here, it's 0; unless I use 'read' on cmdline!
ec="$?"
echo
echo "What the ec=$ec"
exit "$ec"
