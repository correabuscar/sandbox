#src: https://lists.gnu.org/archive/html/bug-gawk/2022-09/msg00057.html
#ie. heftig's simple fix altho fixed that, causes this new bug, which is fixed by ^
#awk: afterthesimplefix_newbug.awk:6: fatal: internal error: file interpret.h, line 254: unexpected parameter type Node_val
function foo(x)
{
        if (x == int(x))
                return (int(x) != 0)
}
BEGIN {
        foo(P["bar"])
}
