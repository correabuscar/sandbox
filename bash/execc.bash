echo 'a'
exec 1>&2
echo 'ok, executed only once - shows "a" only once'
