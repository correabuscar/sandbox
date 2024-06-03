#!/bin/bash

# Find files and directories with ACLs set
#made by chatgpt-4o

# Path to search
SEARCH_PATH="/"

#find "$SEARCH_PATH" -exec getfacl --absolute-names --skip-base {} + 2>/dev/null | \
#grep -B 1 "^# file: " | \
#grep -v "^# file: " | \
#grep -v "^--$" | \
#sed 's/^# file: //'

find "$SEARCH_PATH" -print0 2>/dev/null | \
xargs -0 -P0 -- getfacl --absolute-names --skip-base 2>/dev/null | \
awk 'BEGIN { prev=""; } /^# file:/ { prev=substr($0, 9); next } !/^$/ { if (prev!="") print prev; prev="" }' | \
sort | uniq
#awk 'BEGIN { FS=""; OFS=""; } /^# file:/ { if (getline > 0 && $0 !~ /^$/) print prev; prev = substr($0, 9); next } { prev = "" }' | \
#grep -B 1 "^# file: " | \
#grep -v "^# file: " | \
#grep -v "^--$" | \
#sed 's/^# file: //'

