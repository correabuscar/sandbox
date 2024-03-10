#initial code from src: https://stackoverflow.com/a/68018445

cmake_minimum_required(VERSION 3.28.3)
# My dummy data.
file(READ "file.log" file_content)
# replace idea from https://gitlab.kitware.com/cmake/cmake/-/issues/19156#note_557314
# problem we're trying to avoid: https://cmake.org/cmake/help/v3.0/manual/cmake-language.7.html#lists https://cmake.org/cmake/help/v3.0/manual/cmake-language.7.html#bracket-argument 
string(REPLACE "?" "?0" file_content "${file_content}") #must be first
string(REPLACE "[" "?1" file_content "${file_content}")
string(REPLACE "]" "?2" file_content "${file_content}")
#string(REPLACE "[" unique1 file_content "${file_content}" REPLACE "]" unique2 file_content "${file_content}")

#the following is happening in: Modules/CMakeParseImplicitIncludeInfo.cmake eg. https://gitlab.kitware.com/cmake/cmake/-/commit/4b46523d905451ebdcf0ef8476ebe875945b3a62#65085631ca659cd8b5fc4c3d02fb1efa7b712f4e_171_170
string (REGEX REPLACE "\r*\n" ";" _my_list "${file_content}")
foreach (line IN LISTS _my_list)

  #restore original line: (note that original ";" are gone, only escaped "\;" in original are kept as escaped)
  # WORKAROUND: we have to replace because `foreach(... IN LISTS ...)` discardes ;-escaping
  #string(REPLACE ";" "\;" line "${line}")
  string(REPLACE "?2" "]" line "${line}")
  string(REPLACE "?1" "[" line "${line}")
  string(REPLACE "?0" "?" line "${line}") #must be last
  message("_my_list has '${line}'")

endforeach()

function(my_function)
  #set(local_var "Funct;\;ion-local variable" PARENT_SCOPE)
  set(local_var "Funct;\;ion-local variable")
  message("Inside function: ${local_var}")  # This will print "Function-local variable"
endfunction()

my_function()  # Call the function
message("Outside function: ${local_var}")  # This will print an empty string

