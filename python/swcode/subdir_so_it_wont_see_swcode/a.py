#!/usr/bin/pypy3 -bb


import sys
oldsyspath=sys.path
sys.path.append("/swcode/")
import swcode
sys.path=oldsyspath

#these cases are handled:

#swcode.module.VERSION="a"
#del swcode.module.VERSION
#type(swcode.module).VERSION="a"
#print(type(swcode.module))

#print(swcode.module.__version__)
print(swcode.module.VERSION)
