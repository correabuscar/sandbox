#https://lists.gnu.org/archive/html/bug-gawk/2020-07/msg00004.html
#well this works now, so this isn't it!
BEGIN {
        $0 = "a0 b"
        gsub(/0/, "", $1)
        $2 == "b"
        $0 = ""
}
