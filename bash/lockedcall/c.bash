#!/bin/bash

source /swcode/swcode.bash

function moo() {
  echo "moo_start $*"
  sleep 1
  echo "moo_end $*"
}

lockedcall 300 moo c &
wait
echo "c done."
