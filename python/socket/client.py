#!/usr/bin/pypy3 -bb

#src: https://docs.python.org/3/library/socket.html#example
# Echo client program
import socket, time

HOST = ''    # The remote host
PORT = 57              # The same port as used by the server
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b'Hello, world')
    data = s.recv(1024)
    time.sleep(10)
print('Received', repr(data))
