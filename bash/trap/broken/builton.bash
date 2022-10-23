#!/bin/bash

#note: got some info from <earnestly> #bash freenode: https://www.cons.org/cracauer/sigint.html

#trap SIGINT won't return proper exit code if interrupted command was builtin!

interrupted() {
	local ec="$?"
	trap - SIGINT  #this restores prev. behaviour, so now another C-c will keep the exit code from 'read -rp'!
  echo "interrupted sees exit code '$ec'"
#  exit "$ec"  #this is needed ONLY for when 'sleep' is used and interrupted, or else it will continue running and hit 'Normal exit'
}

trap interrupted SIGINT
#echo "Press C-c here"; sleep 100 # press C-c here, XXX why is exit code 130 correctly detected here,
read -rp "Press C-c here..." # but not here, it's 0(probably because it doesn't interrupt 'read' at all); unless I press C-C twice, or use this 'read' on cmdline and press C-c once! note: 'read' is builtin and 'sleep' is '/usr/bin/sleep'
ec="$?"
echo
echo "Normal exit sees ec=$ec"
exit "$ec"
