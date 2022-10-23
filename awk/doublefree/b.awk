#originally: $ /var/tmp/portage/sys-apps/gawk-5.2.0/work/gawk-5.2.0/gawk -f  /usr/src/linux-5.18.19-gentoo-r1/arch/x86/tools/gen-insn-attr-x86.awk  /usr/src/linux-5.18.19-gentoo-r1/arch/x86/lib/x86-opcode-map.txt
#
#gentoo bug: https://bugs.gentoo.org/868567
#free(): double free detected in tcache 2
#
function add_flags(old) {
  if (old)
    return 0
  if (!old)
    return 1
}
BEGIN {
  a[0]=add_flags(a[0])
}
