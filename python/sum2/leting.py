#!/usr/bin/python3
#see: https://www.youtube.com/watch?v=XKu_SEDAykw

#code from cuspctrl.py and swcode.py


from typing import Any,Tuple
import sys,os

##-----
#import sys #i know it's already imported, but just making sure this whole block can be moved before the prev. import, if ever.
#oldsyspath=sys.path
#sys.path.append("/swcode/")
#import swcode #vim's lexer for .py cannot see this so not using it this way then.
#sys.path=oldsyspath
##-----

#XXX: using underscore prefix so that 'from swcode import *' (which is discouraged) won't import these too.

#src: https://stackoverflow.com/questions/100003/what-are-metaclasses-in-python/35732111#35732111
class _ROValidator(type):  # a metaclass!
    #def __new__(metacls, cls, bases, clsdict):
    #    metacls.__slots__=()
    #    return super().__new__(metacls, cls, bases, clsdict)
    #__setattr__ src: https://stackoverflow.com/questions/100003/what-are-metaclasses-in-python/21999253#21999253
    def __setattr__(self, name:str, value:str) -> None:
        #print(type(cls), type(name), type(value))
        raise AttributeError(f"Won't set read-only constant '{name}' of class '{self}' to value '{value}'")
    def __delattr__(self, name:str) -> None:
        #print(type(name))
        #detects deletion of the class who has self as metaclass, not deletions of attributes within this metaclass.
        raise AttributeError(f"1Attempted to delete attribute '{name}' of class '{self}', perhaps in order to defeat read-only-ness of constants?")

#__slots__ src: https://stackoverflow.com/questions/2682745/how-do-i-create-a-constant-in-python/23274028#23274028
#class _Module(object):
class Constants(metaclass=_ROValidator):
#class _Module(metaclass=_ROValidator):
    __slots__ = () #this makes inst.VERSION="2" yield: AttributeError: '_Module' object attribute 'VERSION' is read-only
    #VERSION=ValidateType("0.0.8")
    #VERSION="0.0.10"
    # ^ this is swcode module's version
    #def __init__(self):
    #    a=super().__init__()
    #    self.__slots__=()
    #    return a
    #    #del self.__dict__
    #    #del self.__weakref__
    #def __dir__(self):
    #    a = super().__dir__()
    #    a.remove("__slots__")
    #    a.remove("__dict__")
    #    #no effect
    #    return a
    def __init_subclass__(cls) -> None:#, /, **kwargs):
        #if hasattr(cls,'__dict__'): #always true hmm
        #if cls.__dict__: #same
        #print(cls.__dir__)
        if "__dict__" in dir(cls): #good!
            raise SyntaxError(f"failed! __dict__ shouldn't exist! Make sure you add the following line to your subclass(which is '{cls}'): '__slots__ = ()' (there's no way to inherit it or similar, so you must add it by hand)")
        super().__init_subclass__()
        return #apparently doesn't return any value, see: https://docs.python.org/3/reference/datamodel.html#object.__init_subclass__
    def __delattr__(self, name:str) -> None:
        raise AttributeError(f"2Attempted to delete attribute '{name}' of class '{self}', perhaps in order to defeat read-only-ness of constants?")


#src: https://stackoverflow.com/questions/287871/how-to-print-colored-text-in-terminal-in-python/287944#287944
#class bcolors: #b doesn't mean background
class _BColors(Constants):
    __slots__ = () #required for Constants

    #HEADER = '\033[95m'
    #OKBLUE = '\033[94m'
    #OKGREEN = '\033[92m'
    RED = '\033[91m'
    BG_RED="\x1B[41m" #tput setab 1
    BG_DARKRED="\x1B[48;5;52m" #tput setab 52
    FAIL = '\033[41m'
    WARN = '\033[38;5;91m'
    #ENDC = '\033[0m'
    ENDC = '\033(B\033[m' #aka reset aka `tput sgr0`
    RESET="\x1B(B\x1B[m" #tput sgr0, in xfce4-terminal, TERM=xterm-256color
    #BOLD = '\033[1m'
    #UNDERLINE = '\033[4m'
    DEBUG = '\033[90m'
    BG_GREEN="\x1B[42m" #tput setab 2
#doneTODO: add the other colors eg. color_green to this list and use this method! (forgot about this due to months of break from coding this)

bcolors=_BColors()
del _BColors #because we should use the instance, instead.

class _Const_pconsts(Constants):
    __slots__=() #required
    DOLLAR0:str = os.path.realpath(__file__) #aka $0 in bash, ie. `realpath -- "$0"`  unless this code resides inside a python module!
CONST = _Const_pconsts()  #don't forget these parens! else you get <property object at 0x00007f4a83140920> when referencing the constants!
del _Const_pconsts

#PROGRAM_VERSION: Final = "0.0.26"

def colored(color: str, text: str) -> str:
    return f"{color}{text}{bcolors.ENDC}"

#takes any number or args
def debugmsg(*anything:Any) -> str:
    #return colored(bcolors.DEBUG,"Debug: "+ str(anything))
    return colored(bcolors.DEBUG,"Debug: "+ ' '.join(str(element) for element in anything))

def eprint(*args: Any, **kwargs: Any) -> None:
    print(*args, file=sys.stderr, **kwargs)

def debugpr(*args: Any, **kwargs: Any) -> None:
    #XXX: I don't want to put the "if __debug__:" inside this! because I want this to be callable from release (aka -OO arg of pypy3)
    eprint(debugmsg(*args), **kwargs)


#------------------------
def way1(zlist,zsum)-> Tuple[bool,Tuple[int,int]]:
    ret=False
    print(zlist,f" sum={zsum}")
    items=len(zlist)
    for i in range(0,items):
        for j in range(i+1,items):
            if (zlist[i]+zlist[j]==zsum):
                ret=True
            print(f"is {zlist[i]}+{zlist[j]}={zsum} ? {ret}")
            if ret:
                break
    return (ret,(zlist[i],zlist[j]))

#TODO: make the other ways!

def main() -> int:
    l1=[1,2,4,9]
    l2=[1,2,4,4]
    zsum=8
    print(way1(l1,zsum))
    print(way1(l2,zsum))
    return 0
#------------------------

if __name__ == '__main__':
    if __debug__:
        debugpr("Running main() of '{self}".format(self=CONST.DOLLAR0))
    main()
    if __debug__:
        debugpr("Done main() of '{self}".format(self=CONST.DOLLAR0))
else:
    if __debug__:
        debugpr("Imported '{self}' as module.".format(self=CONST.DOLLAR0))

