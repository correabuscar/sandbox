#!/usr/bin/pypy3 -bb

#src: https://www.tutorialspoint.com/enum-in-python
import enum
class module(enum.Enum):
   someint=3
   __version__ = "0.0.1"
   __ver = __version__ #prints as "module._module__ver" via 'for'
   __ver2 = "uniq?"
   #_version_ = __version__ #ValueError: _names_ are reserved for future Enum use
   somethingelse = "2222"
   VERSION = __version__
#on using "__" as prefix: "deltab: right, that triggers a form of name mangling"

print(module.__version__)
#print(module.__ver)
#print(module.__ver2) #  File "/opt/pypy3/lib-python/3/enum.py", line 326, in __getattr__
#    raise AttributeError(name) from None
#AttributeError: __ver2

#print(module._version_)
print(module.VERSION)
print(module.somethingelse)
print(module.someint)

for each in (module):
    print(each.value)
