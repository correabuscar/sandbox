#!/usr/bin/env -S PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/bin:${PATH}" /run/current-system/sw/bin/bash

echo "$# $@" >>/tmp/linker.cmds
#/run/current-system/sw/bin/rustc
"$@"
