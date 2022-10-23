#!/bin/bash

for i in 1 2 3 4; do
  echo 'a' & #required
  read -r -t 0.1 -s -- &
  #echo "$?" #is 0
done
#mejobs=($(jobs -p))
#wait -f "${mejobs[@]}"
wait
