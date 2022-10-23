#!/bin/bash

a() {
  return "$1"
}

b() {
  local R_DEPS_SATISFIED=0
  local R_DEPS_MISSING=1
  deplist="a"
  [[ -z $deplist ]] && return $R_DEPS_SATISFIED
  return $R_DEPS_MISSING
}

#a 0 || echo "anope"
b || echo "error"


