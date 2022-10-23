#!/usr/bin/pypy3 -bb

#src: https://stackoverflow.com/questions/2959474/making-a-python-script-only-be-able-to-run-once-at-a-time/7256476#7256476

import os
import sys
import fcntl

import time

#fh=0
def run_once():
    #global fh
    try:
        f=os.path.realpath(__file__)
        #f="/tmp/a"
        if __debug__:
            print("Using lock file '{f}' to prevent this script from running more than once!".format(f=f))
        fh=open(f,'r')
    except: # Exception as e:
        #print("open failed",e)
        print("open lock file failed")
        raise
        #os._exit(1)
    try:
        fcntl.flock(fh,fcntl.LOCK_EX|fcntl.LOCK_NB)
    except:
        print("already running")
        os._exit(2)
    print("first time, running alone (now try running it again from another terminal)")
    time.sleep(5)

run_once()
print("exiting")
