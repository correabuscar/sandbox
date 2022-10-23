#!/usr/bin/python3

print(ord('H'))
print(ord('❦'))

#src: /home/user/sandbox/python/sha256/SHA256-PYTHON/Utils/utils.py
def AND(i, j): return [and_(ia, ja) for ia, ja in zip(i,j)]
def and_(i, j): return if_(i, j, 0)
def isTrue(x): return x == 1
def if_(i, y, z): return y if isTrue(i) else z

#print(AND([1,0,1],[0,1,0]))
