#!/bin/bash

a() {
  echo $#
  echo "$@"
}

#FIXED version: arr=()
arr=
#^ unfixed version, don't do this!

a "${arr[@]}" 1 2 3
#^ 4 args, incorrect
a ${arr[@]} 1 2 3
#^ 3 args, correct

arr=("4_1 4_2" 6)
a "${arr[@]}" 1 2 3
#^ 5 args, correct!
a ${arr[@]} 1 2 3
#^ 6 args, INcorrect!

