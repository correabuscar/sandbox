#!/usr/bin/env /run/current-system/sw/bin/bash

echo "$# $@" >>/tmp/rustc.cmds
#/run/current-system/sw/bin/rustc
"$@"
