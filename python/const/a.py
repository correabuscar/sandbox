#!/usr/bin/python
def constant(f):
    def fset(self, value):
        raise TypeError
    def fget(self):
        return f()
    return property(fget, fset)

#def constant2(f):
#    def fset(self, value):
#        raise TypeError
#    def fget(self):
#        return f()
#    return class(fget,fset)

#@constant
#@constant2
class _Const(object):
    @constant
    def FOO():
        return 0xBAADFACE
    @constant
    def BAR():
        return 0xDEADBEEF
    def moo():
        pass

CONST = _Const()

#print CONST.FOO
##3131964110

#CONST.FOO = 0
##Traceback (most recent call last):
##    ...
##    CONST.FOO = 0
##TypeError: None

print(CONST.FOO) # 3131964110
print(type(CONST.FOO))
def foo(): pass
print(CONST.FOO.__class__)
print(foo.__class__)
print(CONST.moo.__class__)
type(CONST).FOO=1  #mypy says "error: Cannot assign to a method"  #but it still works and thus the readonly stuff is a fail. Thanks <_habnabit> on freenode irc #python
_Const.FOO=2  #same thing
print(CONST.FOO) # 1
#TODO: can use something like this[1] to prevent assignment, maybe: [1] https://stackoverflow.com/questions/100003/what-are-metaclasses-in-python/35732111#35732111


#src: https://stackoverflow.com/questions/2682745/how-do-i-create-a-constant-in-python/23274028#23274028
class CONST(object):
    __slots__ = ()
    FOO = 1234

CONST = CONST()
print(CONST.FOO)    # 1234

#CONST.FOO = 4321              # AttributeError: 'CONST' object attribute 'FOO' is read-only
#CONST.__dict__['FOO'] = 4321  # AttributeError: 'CONST' object has no attribute '__dict__'
#CONST.BAR = 5678              # AttributeError: 'CONST' object has no attribute 'BAR'

type(CONST).FOO=1
print(CONST.FOO)    # 1


