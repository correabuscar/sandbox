#!/bin/bash

stty
for i in 1 2 3 4; do
  echo 'a' & #required
  read -r -t 0.1 -s -- &
  #echo "$?" #is 0
done
wait
stty
stty echo echok
