cmake_minimum_required(VERSION 3.28.3)
#cmake_minimum_required(VERSION 2.28)
#code from src: https://gitlab.kitware.com/cmake/cmake/-/issues/19156#note_557314
function(test_file_content file_path)
  file(STRINGS "${file_path}" file_content)

  # WORKAROUND: we have to replace because `file(STRINGS` does a break on not closed `]` or `[` characters
  string(REPLACE "?" "?0" file_content "${file_content}")
  string(REPLACE "[" "?1" file_content "${file_content}")
  string(REPLACE "]" "?2" file_content "${file_content}")

  foreach(line IN LISTS file_content)
    # WORKAROUND: we have to replace because `foreach(... IN LISTS ...)` discardes ;-escaping
    #string(REPLACE ";" "\;" line "${line}") #FIXME: doesn't seem to be needed?

    #string(REPLACE "?0" "?" line "${line}") #too soon doneFIXME:
    string(REPLACE "?1" "[" line "${line}")
    string(REPLACE "?2" "]" line "${line}")
    string(REPLACE "?0" "?" line "${line}") #should be last, else bugged
    message("${line}")
  endforeach()
endfunction()

test_file_content("erm.log")
