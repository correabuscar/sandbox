#!/bin/bash

a(){
  echo "a '$0'"
  b() {
    echo "b '$0'"
  }
  b
  echo "aa '$0'"
}

a
