#
#regex='[[:alnum:]]+|[^[:alnum:][:space:]]|[[:space:]]'
regex='[[:alnum:]]+'
regex='.'
#regex='[[:space:]]+|[[:alnum:]]+|[^[:alnum:][:space:]]|[[:space:]]'
git diff -U1 --no-index --word-diff=plain --word-diff-regex="$regex" --color=always  {9,8}.log | cat 
#FIXME: the 'added line1' shows concatenated to prev. line!! but it should be on next line alone!
git diff -U1 --no-index {9,8}.log | cat
