#initial code from src: https://stackoverflow.com/a/68018445

cmake_minimum_required(VERSION 3.28.3)
# My dummy data.
set (foo "foohere")
set (_my_list "1 ] A B [ C D ) E F ( G H } I { J \${foo} 1 ] 2 3 [[ some thing ]] 4 5 6")
string (REPLACE " " ";" _my_list "${_my_list}")

# Actual looping.
foreach (my_entry IN LISTS _my_list)
#foreach (my_entry ${_my_list}) #same

    # All done! do something with ${my_entry}, for example:
    message("_my_list has '${my_entry}'")

endforeach()

foreach(arg
    NoSpace
    Escaped\ Space
    This;Divides;Into;Five;Arguments
    Escaped\;Semicolon
    ]
    shie
    ];da
    [;du
    a;b;];c;[;d;e;f;[;g;h;];i;
    aaa[sh[i]e];bbbb
    aaa\[[sh\[[i\]]e\]];bbbb
    )
  message("${arg}")
endforeach()
  message([==[
This is the first line in a bracket argument with bracket length 1.
No \-escape sequences or ${variable} references are evaluated.
This is always one argument even though it contains a ; character.
The text does not end on a closing bracket of length 0 like ]=].
[][][[][
It does end in a closing bracket of length 1.
]==])
