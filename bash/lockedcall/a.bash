#!/bin/bash

source /swcode/swcode.bash

function moo() {
  echo "moo_start $*"
  sleep 1
  echo "moo_end $*"
}

lockedcall 300 moo 300_1 &
lockedcall 300 moo 300_2 &
lockedcall 300 moo 300_3 &
lockedcall 301 moo 301_1 &
lockedcall 301 moo 301_2 &
#works as you'd expect: the 300s group are in series, the 301s same, but 300s and 301s will go in parallel
#moo_start 300_1
#moo_start 301_1
#moo_end 300_1
#moo_end 301_1
#moo_start 300_2
#moo_start 301_2
#moo_end 300_2
#moo_end 301_2
#moo_start 300_3
#moo_end 300_3

wait
echo "all done."
