#!/bin/bash

source /swcode/swcode.bash

set > "/tmp/moo.$(id -u).log"
rerunasroot "$@"
