#
regex='[[:alnum:]]+|[^[:alnum:][:space:]]|[[:space:]]'
regex='[[:alnum:]]+'
regex='.'
#regex='[[:space:]]+|[[:alnum:]]+|[^[:alnum:][:space:]]|[[:space:]]'
git diff -U1 --no-index --word-diff=plain --word-diff-regex="$regex" --color=always -- <(echo -e "a b c\nNEWLINE1\nNEWLINE2\nENDLINE") <(echo -e "a F c\n\nENDLINE") | cat 
#FIXME: the 'added line1' shows concatenated to prev. line!! but it should be on next line alone!
