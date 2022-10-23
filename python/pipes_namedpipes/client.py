#!/usr/bin/pypy3 -bb

#src: https://stackoverflow.com/questions/33231767/interprogram-communication-in-python-on-linux/33259023#33259023
# client.py

import os

pipe_name = 'pipe_test'

if not os.path.exists(pipe_name):
    os.mkfifo(pipe_name)

print("Waiting for server to start...")
with open(pipe_name, 'w') as pipe:
    action = input("Enter action to send: ")
    pipe.write(action + '\n')
