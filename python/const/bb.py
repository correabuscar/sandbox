#!/usr/bin/python


class _CONST:
    __slots__ = ()
    #def __init__(self):
    #    super().__init__()
    #    self.__slots__=()
    #    #del self.__dict__
    def __init_subclass__(cls):#, /, **kwargs):
        #if hasattr(cls,'__dict__'): #always true hmm
        #if cls.__dict__: #same
        if "__dict__" in dir(cls): #good!
            print("failed! __dict__ shouldn't exist!")
        #print(cls, dir(cls))
        #del cls.__slots__
        cls.__slots__=() #no effect
        super().__init_subclass__() #**kwargs)
        cls.__slots__=() #no effect
    def __new__(cls):
        cls.__slots__=() #no effect
        #del cls.__slots__
        #del cls.__dict__ #not writable!
        #cls.__dict__=""
        #print(cls,dir(cls))
        inst=super().__new__(cls)
        inst.__slots__=() #no effect
        #del inst.__slots__
        #del inst.__dict__
        return inst

class C(_CONST):
    #__slots__ = () #XXX: only works when this line is present! but want to inherit it instead!
    #__dict__=()
    #def __init__(self):
    #    super().__init__()
    #    del self.__slots__
    VER="0.0.1"

inst=C()

good=False
try:
    inst.VER="2"
except AttributeError as e:
    print(e)
    good=True

assert good
print("success")

