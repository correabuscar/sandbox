#initial code from src: https://stackoverflow.com/a/68018445

cmake_minimum_required(VERSION 3.28.3)
# My dummy data.
file(READ "file.log" file_content)
set (_my_list [==[1
11
]
2
3
if [[ some thing ]]
4
5
6
]
7
8
[
9
10
11
[
12
13
]==])
#string (REGEX REPLACE "\r*\n" ";" _my_list "${file_content}")
string (REGEX REPLACE "\r*\n" ";" _my_list "${_my_list}")

# Actual looping.
foreach (my_entry IN LISTS _my_list)
#foreach (my_entry ${_my_list}) #same

    # All done! do something with ${my_entry}, for example:
    message("_my_list has '${my_entry}'")

endforeach()

