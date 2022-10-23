#!/bin/bash

another=("6 7" "with  spaces")
array=( one two three "4 5")
#array+=(${another[@]})  #fail
array+=("${another[@]}")
#for i in "${array[@]}"; do #fail, consecutive spaces reduced to ONE space
#	echo $i
#done

#for id in `seq 0 1 $(( "${#array[@]}" -1 ))`; do #success but too verbose!
#  echo "${array[id]}"
#done

#Found out how to do a normal for loop in bash from here src: https://stackoverflow.com/questions/8880603/loop-through-array-of-strings-in-bash/22432604#22432604
#for (( id=0; id < "${#array[@]}"; id++ )); do
#  echo "${array[id]}"
#done
#echo "$id"//yep, still visible! ==6==arraylen


#nope: for (( local id=0; id < "${#array[@]}"; id++ )); do
a(){
  #( #don't bleed the 'for' var outside this subshell block; still bleeds it if using {} parens instead of subshell () ones
  # two ; used for when tested {} parens
    local id; #works!! won't bleed it outside function!
  for (( id=0; id < "${#array[@]}"; id++ )); do
    echo "${array[id]}"
#    if test "$id" == "0" -o "$id" == "1"; then
#      array+=($id)
#yep, this id < "${#array[@]}" is evaluated on every iteration
#    fi
  done; #)
  echo "id(1)=$id"
}

a
echo "id(2)=$id"

