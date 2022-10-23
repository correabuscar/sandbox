#!/usr/bin/python

class _MCValidator(type):
    def __delattr__(self, name):
        #print(f"5{name}")
        raise AttributeError(f"5Attempted to delete attribute '{name}' of class '{self}', perhaps in order to defeat read-only-ness of constants?")
    def __delete__(self, inst):
        print(f"6{inst}")

class _Validator(type,metaclass=_MCValidator):
    #def __new__(metacls, cls, bases, clsdict):
    #    return super().__new__(metacls, cls, bases, clsdict)
    def __setattr__(cls, name, value):
        raise AttributeError(f"Won't set read-only constant '{name}' of class '{cls}' to value '{value}'")
    def __delattr__(self, name):
        #print(f"1{name}")
        raise AttributeError(f"1Attempted to delete attribute '{name}' of class '{self}', perhaps in order to defeat read-only-ness of constants?")
    def __delete__(self, inst):
        print(f"2{inst}")

class _CONST(metaclass=_Validator):
    __slots__ = ()
    VER="0.0.1"
    def __delattr__(self, name):
        #print(f"3{name}")
        raise AttributeError(f"3Attempted to delete attribute '{name}' of class '{self}', perhaps in order to defeat read-only-ness of constants?")
    def __delete__(self, inst):
        print(f"4{inst}")

#print(type(Validator.__setattr__))
bad=True
try:
    del _Validator.__setattr__  #5
except AttributeError as e:
    print(f"Good3: {e}")
    bad=False
assert not bad

bad=True
try:
    del _CONST.__setattr__  #1
except AttributeError as e:
    print(f"Good4: {e}")
    bad=False
assert not bad

inst=_CONST()

bad=True
try:
    del inst.__setattr__ #3
except AttributeError as e:
    print(f"Good5: {e}")
    bad=False
assert not bad

bad=True
try:
    type(inst).VER="1"
except AttributeError as e:
    print(f"Good1: {e}") #Good1: Won't set read-only constant 'VER' of class '<class '__main__._CONST'>' to value '1'
    bad=False
assert not bad

bad=True
try:
    inst.VER="2"
except AttributeError as e:
    print(f"Good2: {e}") #Good2: '_CONST' object attribute 'VER' is read-only
    bad=False
assert not bad

