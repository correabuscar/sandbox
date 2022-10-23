#!/usr/bin/env bash
#src: https://serverfault.com/a/818671
set -eux
check() {
    #if [[ ${array[@]} ]]; then
    if test -n "${array[@]}"; then
        echo not empty
    else
        echo empty
    fi
}
check   # empty
array=(a b c d)
check   # test: too many arguments

array=()
check   # empty

array="$@" #empty
check
array=("$@") #not empty
check
array=($@) #not empty
check

array=("") # empty
check

array=("" "") #error
check
