#!/bin/bash
#!/home/user/build/1packages/4used/bash-devel-git/makepkg_pacman/bash/src/bash/bash
#!/home/user/build/1packages/4used/bash-devel-git/makepkg_pacman/bash/src/bash/bash --posix
#!/bin/bash --posix

#XXX: it acts ok with this ^ --posix, thanks to Chet Ramey here: https://lists.gnu.org/archive/html/bug-bash/2020-04/msg00110.html  so it also needs the exit and it is exit code 0 still hmm

#C-c won't break the builtin read unless default SIGINT trap is restored, thus needing two C-c to exit

#XXX: now fixed by bash patches: make_read_be_always_posix.patch and get_exitcode_130_on_sigint_read.patch

interrupted() {
	local ec="$?"
	#trap - SIGINT  #this restores prev. behaviour, so now another C-c will stop 'read -rp'
  echo "interrupted sees exit code '$ec'"
  exit "$ec"  #this is needed ONLY for when 'sleep' or 'ping' is used and interrupted, or else it will continue running and hit 'Normal exit', if used with 'read' then it will exit without breaking 'read' thus exit code is 0 instead of 130 (128+2 aka SIGINT==2)
}

trap interrupted SIGINT SIGTERM
#builtin read -e  #damn, that bash_event_hook() in bashline.c really is reached only on 'read -e' not 'read -p'
builtin read -rp "Press C-c here..."  #can use 'sleep 100' instead of this 'read...' here, for testing how the behaviour is different with an external command instead of a builtin one.
#sleep 100
#ping 127.0.0.1  #ping will ec=0 on C-c, thanks to: Greg Wooledge https://lists.gnu.org/archive/html/bug-bash/2020-04/msg00096.html
ec="$?"
echo
echo "Normal exit sees ec=$ec"
exit "$ec"
