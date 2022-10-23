#!/bin/bash

cleanup() {
  echo 'cleanup'
}

secundo() {
  echo 'secundo'
}

trap cleanup EXIT
trap secundo EXIT #overwrites prev. !
