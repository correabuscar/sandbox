#!/usr/bin/env -S PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/bin:${PATH}" bash

# Enable extglob for extended pattern matching
shopt -s extglob

# Test cases
paths=(
    "////tmp///a"
    "tmp/a"
    "tmp///a"
    "/tmp//a/b///c"
    "//var//log///messages"
)

for old_file_path in "${paths[@]}"; do
    # Trim down consecutive slashes using extglob
    #       ${parameter//pattern/string}
    #"If there are two slashes separating parameter and pattern, all matches of pattern are replaced with string."
    #"If the extglob shell option is enabled using the shopt builtin, the shell recognizes  several  extended  pattern
    # matching operators.  In the following description, a pattern-list is a list of one or more patterns separated by
    # a |.  Composite patterns may be formed using one or more of the following sub-patterns:"
    # +(pattern-list)
    #   Matches one or more occurrences of the given patterns
    slimmed_path="${old_file_path//+(\/)/\/}"

    # Print the result
    echo "Original: $old_file_path"
    echo "Slimmed:  $slimmed_path"
    echo
done

