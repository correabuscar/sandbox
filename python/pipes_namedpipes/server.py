#!/usr/bin/pypy3 -bb

#src: https://stackoverflow.com/questions/33231767/interprogram-communication-in-python-on-linux/33259023#33259023
# server.py

import os
import time

pipe_name = 'pipe_test'

if not os.path.exists(pipe_name):
    os.mkfifo(pipe_name)

with open(pipe_name, 'r') as pipe:
    print("Listening for actions...")
    while True:
        action = pipe.readline()[:-1]
        if action == '':
            print("No clients. Sleeping...")
            time.sleep(1)
        else:
            print("Action received:", repr(action))
