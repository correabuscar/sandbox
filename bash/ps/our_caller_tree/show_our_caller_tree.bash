#!/bin/bash

# Array to store process information
declare -a process_tree

# Set to store processed PIDs
declare -A processed_pids

# Function to retrieve process information recursively
build_process_tree() {
  #shopt -s extglob  # Enable extended globbing
    local pid=$1
    local indent=$2
    #echo "New: $# $*"
    local trimmed_pid
    #trimmed_pid="${pid#"${pid%%[![:space:]]*}"}"   # Remove leading whitespace
    #trimmed_pid="${trimmed_pid%"${trimmed_pid##*[![:space:]]}"}"   # Remove trailing whitespace
    trimmed_pid="${pid#"${pid%%[![:space:]]*}"}"   # trim leading whitespace
    trimmed_pid="${trimmed_pid%"${trimmed_pid##*[![:space:]]}"}"   # trim trailing whitespace
    #echo "trim='$trimmed_pid'"




    # Check if we've already processed this PID to avoid infinite loops
    if [[ -n "${processed_pids[$trimmed_pid]}" ]]; then
      echo "!! Avoided infinite loop for '$pid' due to bad coding!" >&2
      return 2
    fi

    # Mark this PID as processed
    processed_pids[$trimmed_pid]=1

    # Read command line arguments from /proc/PID/cmdline into an array
    local cmdline_args=()
    while IFS= read -r -d $'\0' arg; do
        cmdline_args+=("$arg")
    done < "/proc/$trimmed_pid/cmdline"

    # Determine the command (first argument) and its arguments count
    #local cmd="${cmdline_args[0]}"
    local args_count="${#cmdline_args[@]}"

    ## Get process command line
    #local cmd
    ##cmd="$(ps -o cmd= -p "$pid" 2>/dev/null)"
    #cmd="$(ps -o cmd= -p "$trimmed_pid")" # 2>/dev/null)"

    if test "$args_count" -gt "0"; then
        # Store process information in the array
        #process_tree+=("${indent}PID: '$pid' - Command: '$cmd'" )
        #process_tree+=("${indent}'$cmd'" )
        local all_args=""
        for element in "${cmdline_args[@]}"
        do
          # Enclose each element in single quotes and concatenate with the existing out_str
          all_args="${all_args}'${element}' "
        done
        all_args="${all_args% }"
        #used $pid with spaces on purpose - doesn't apply anymore, $pid is trimmed too.
        process_tree+=("${indent}'$pid'-'$args_count'-${all_args}")

        # Find parent PID
        local ppid
        #ppid="$(ps -o ppid= -p "$pid" 2>/dev/null)"
        #ppid="$(ps -o ppid= -p "$trimmed_pid" 2>/dev/null)"
        ppid="$(awk '{print $4}' "/proc/$pid/stat")"
        if test "$ppid" != "0"; then
          local trimmed_Ppid
          trimmed_Ppid="${ppid#"${ppid%%[![:space:]]*}"}"   # Remove leading whitespace
          trimmed_Ppid="${trimmed_Ppid%"${trimmed_Ppid##*[![:space:]]}"}"   # Remove trailing whitespace
          #echo "trim2='$trimmed_Ppid'"


        # Recursively call function for parent process
        #if [ "$trimmed_Ppid" != "1" ] && [ "$trimmed_Ppid" != "$trimmed_pid" ]; then
            build_process_tree "$ppid" "$indent  "
        fi
    fi
}

# Start with our own PID
build_process_tree "$$" ""

# Print the process tree
str="Our callers tree:"$'\n'
for line in "${process_tree[@]}"; do
    #echo "$line"
    str+="${line}"$'\n'
done
echo -n "$str"
