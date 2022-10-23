#!/bin/bash

mkdir -p destdir sourcedir/a
touch sourcedir/file1 sourcedir/a/file2
echo 'sourcedir' >/tmp/filesfrom.list.tmp
echo '//sourcedir//a//////file2' >/tmp/excluded.list.tmp #this won't work unless rsync gets patched
#echo '/sourcedir/a/file2' >/tmp/excluded.list.tmp #yes this will work: [sender] hiding file sourcedir/a/file2 because of pattern /sourcedir/a/file2
rsync --recursive --perms --delay-updates --files-from=/tmp/filesfrom.list.tmp --exclude-from=/tmp/excluded.list.tmp --delete-excluded --debug=FILTER1 -- ./ ./destdir/
if test -r 'destdir/sourcedir/a/file2'; then
  echo 'test failed'
else
  echo 'test succeeded'
fi

#cleanup
rm -rf -- destdir sourcedir 
rm -- /tmp/excluded.list.tmp /tmp/filesfrom.list.tmp
