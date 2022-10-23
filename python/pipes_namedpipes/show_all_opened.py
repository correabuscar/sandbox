#!/usr/bin/python3

#XXX: won't work with pypy3: ModuleNotFoundError: No module named 'psutil'
#!/usr/bin/pypy3 -bb

import psutil #local/python-psutil 5.7.0-1 in ArchLinux
#try:
for proc in psutil.process_iter():
    listt=proc.open_files() #Return regular files opened by process ... XXX so, can't see pipes!
    for po in listt:
        #if po.path=="/home/user/sandbox/python/pipes_namedpipes/pipe_test":
        print(proc.pid,po.path)
#except PermissionError:
#    pass
    print(proc.connections())  #no pid!
print(psutil.net_connections())  #got pids!
