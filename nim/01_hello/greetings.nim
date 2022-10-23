# src: https://nim-lang.org/docs/tut1.html#introduction
# This is a comment

const someelse {.intdefine.}: int = 5  #overridden by -d:someElsE=intvalue (the name is case insensitive)
echo someelse

#actual definition of 'release'(somewhere inside nim) is (guessed):
##const release {.booldefine.}: bool = false
#"The syntax -d:flag is actually just a shortcut for -d:flag=true." src: https://nim-lang.org/docs/manual.html#implementation-specific-pragmas-compileminustime-define-pragmas

var CompileType: string
if defined(release):
  CompileType="release"
else:
  CompileType="debug"

echo CompileType, " compile type has been used."

when nimvm:
  echo "executed at compile time" #XXX: never shows hmm, well how could it?!
  assert 1==2 #no effect
else:
  echo "executed at runtime"

echo "What's your name? "
var name: string = readLine(stdin)
echo "Hi, ", name, "!"

proc someProcThatMayRunInCompileTime(): bool =
  when nimvm:
    # This branch is taken at compile time.
    result = true
  else:
    # This branch is taken in the executable.
    result = false
  #return aka 'return result' is implied! "As all variables, result is initialized to (binary) zero:" src: https://nim-lang.org/docs/manual.html#statements-and-expressions-return-statement

const ctValue = someProcThatMayRunInCompileTime()
let rtValue = someProcThatMayRunInCompileTime()
assert(ctValue == true) #yeah this works!
assert(rtValue == false)
#assert 1==2

when sizeof(int) == 2:
  echo "running on a 16 bit system!"
elif sizeof(int) == 4:
  echo "running on a 32 bit system!"
elif sizeof(int) == 8:
  echo "running on a 64 bit system!"
else:
  echo "cannot happen!"
