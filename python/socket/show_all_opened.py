#!/usr/bin/python3

#XXX: won't work with pypy3: ModuleNotFoundError: No module named 'psutil'
#!/usr/bin/pypy3 -bb

import psutil #local/python-psutil 5.7.0-1 in ArchLinux
import pprint
#pp = pprint.PrettyPrinter(indent=4, compact=True, width=10)
#pprint.pprint(psutil.net_connections())
#pprint.pprint(repr(psutil.net_connections()), indent=4, width=1)
for each in psutil.net_connections():  #got pids!
    #each=repr(each)
    pprint.pprint(each, width=1, indent=4)
