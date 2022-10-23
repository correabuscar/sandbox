#!/usr/bin/pypy3 -bb

#src: https://docs.python.org/3/library/socket.html#example
# Echo server program
import socket

HOST = ''                 # Symbolic name meaning all available interfaces
PORT = 57              # Arbitrary non-privileged port
#using 57 to require root
#Running an example several times with too small delay between executions, could lead to this error:
#
#OSError: [Errno 98] Address already in use
#
#This is because the previous execution has left the socket in a TIME_WAIT state, and can’t be immediately reused.
#
#There is a socket flag to set, in order to prevent this, socket.SO_REUSEADDR:
#
#s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
#s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
#s.bind((HOST, PORT))
#
#the SO_REUSEADDR flag tells the kernel to reuse a local socket in TIME_WAIT state, without waiting for its natural timeout to expire.
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind((HOST, PORT))
    s.listen(1)
    conn, addr = s.accept()
    with conn:
        print('Connected by', addr)
        while True:
            data = conn.recv(1024)
            if not data: break
            conn.sendall(data)

