cat $0
exit 0

$ read -r -t 0.1 -s -e -- &
[1] 84663

[1]+  Stopped                 read -r -t 0.1 -s -e --
-----------
user@i87k 2022/10/12 13:23:27 bash5.1.16 t:10 j:1 d:3 pp:84567 p:84570 ut8216
!38931 2 0  5.18.19-gentoo-r1-x86_64 #1 SMP Mon Oct 3 08:29:54 CEST 2022
/home/user 
$ fg
read -r -t 0.1 -s -e --
bash: read: error setting terminal attributes: Interrupted system call

malloc: unknown:0: assertion botched
free: underflow detected; magic8 corrupted
last command: read -r -t 0.1 -s -e -- &
Aborting...Aborted (core dumped)


$ bash --version
GNU bash, version 5.1.16(1)-release (x86_64-pc-linux-gnu)
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>

This is free software; you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

app-shells/bash-9999::localrepo was built with the following:
USE="bundled-readline mem-scramble net (readline) -afs -bashlogger -examples -nls -plugins -vanilla" ABI_X86="(64)"

