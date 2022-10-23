#!/usr/bin/env python
#^ The shebang, or hash-bang, is a special kind of comment which the system uses to determine what interpreter to use to execute the file. The shebang must be the first line of the file, and start with " #! ". - wikipedia

#test what happens when this $0 is suid root whilst using env as shell.
import os
import pwd
import sys
uid=os.getuid()
euid=os.geteuid()
pwuid=pwd.getpwuid( uid ).pw_name
pweuid=pwd.getpwuid( euid ).pw_name

print(f"Hi, uid={uid} euid={euid} pwuid={pwuid} pweuid={pweuid}")
print(f"Python path={sys.path}")
print(f"System PATH={os.environ['PATH']}")
