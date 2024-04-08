#!/usr/bin/env /run/current-system/sw/bin/bash

echo "$# $@" >>/tmp/linker.cmds
#/run/current-system/sw/bin/rustc
"$@"
