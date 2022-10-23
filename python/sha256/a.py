#!/usr/bin/python3

#string src: https://medium.com/@pskib00/hi-i-have-found-out-that-this-string-is-giving-invalid-output-def1f9899a9a?source=responses-----924f660c5d57----0----------------------------
TST_STRING=b"ibsnqwpzhillptcinmtvamymvixjxaumjddwxsxxjhjhnftynajhsluuctgjytazlcdewsexbjcpumdcfbbbmzwxcmjmnxfqurvaarapdswyatlyvqsxdefmehicwwdnkshzgysaxxenmtpirbhphxyaesgwigdxzqpekouenexqkqgpnzzwyjppc"

import hashlib

#This is what currently exists:
m = hashlib.sha256()
print(m.hexdigest())  #e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
m.update(b"Hello ")
m.update(b"World!")
m2 = hashlib.sha256()
m2.update(b"Hello World!")
print(m.hexdigest())  #7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069
print(m2.hexdigest()) #7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069
assert m.hexdigest() == m2.hexdigest()

m3=hashlib.sha256()
m3.update(TST_STRING)
print(m3.hexdigest())

#using trick from: https://stackoverflow.com/a/20749411/19999437
import sys,os
#sys.path.append(os.path.relpath("../SHA256-PYTHON")) #won't work
#sys.path.append(os.path.abspath("/home/user/sandbox/python/sha256/SHA256-PYTHON")) #works
#^ that SHA256-PYTHON is from https://medium.com/@domspaulo/python-implementation-of-sha-256-from-scratch-924f660c5d57 and https://github.com/pdoms/SHA256-PYTHON.git
#XXX: using 'append' here will actually use ./hash.py which is not a module so it will run it.
#using prepend src: https://stackoverflow.com/a/31608399/19999437
sys.path.insert(0,os.path.abspath("/home/user/sandbox/python/sha256/SHA256-PYTHON")) #works, ignored ./hash.py and uses the one in ./SHA256-PYTHON/ dir
from hash import sha256  #vim always shows error on this!
print(sha256(TST_STRING.decode("utf-8")))


m4=hashlib.sha256()
print(m4.hexdigest()) # e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
m4.update(b"a")
print(m4.hexdigest()) # ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
#m4.backwardsupdate(b"a") #can this be done? #for: https://stackoverflow.com/questions/73787368/how-can-the-sha256-hash-be-checked-to-match-against-the-reverse-input-bytes-in
print(m4.hexdigest()) # e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

m5=hashlib.sha256()
heart="❦"
m5.update(heart.encode("utf-8")) # 89be13c35a80c67637c3d793d7fe8db2b6a4d16ea4e73e724d9d01287276cc7
#m5.update(heart.encode()) # 89be13c35a80c67637c3d793d7fe8db2b6a4d16ea4e73e7240d9d01287276cc7
#m5.update(heart.encode('utf16')) # 3a3158502e2ca923034fcc22fe211f888864aaae211d9923768036f60fbf162f
#m5.update(heart.encode('utf-16')) # 3a3158502e2ca923034fcc22fe211f888864aaae211d9923768036f60fbf162f
#m5.update(heart.encode('utf-16-be')) # 83326979189a8336d9c47da00c41dff4f7a4aa5e79fd6ecbdd2d0b2ae7a1bfe2
#m5.update(heart.encode('utf-16-le')) # 2f4583c8b7c73ba0e145d09327d63db85c1b6c4eed2b833aea96d2bc256b7cb6
print(m5.hexdigest())
print(sha256(f"{heart}")) # a9af3ea0eb3881b287b0a243b1444d75078a8ce5acf4522124bb85db27c35ec5
#83326979189a8336d9c47da00c41dff4f7a4aa5e79fd6ecbdd2d0b2ae7a1bfe2
#print(sha256("'f")) # 83326979189a8336d9c47da00c41dff4f7a4aa5e79fd6ecbdd2d0b2ae7a1bfe2


def translate(message,enc='utf-8'):
    for c in message.encode(enc):
        #o=ord(c)
        o=c
        print(o)
        charcodes = [o]
    bytes = []
    for char in charcodes:
        print(char, bin(char),bin(char)[2:],bin(char)[2:].zfill(8))
        binary=bin(char)[2:].zfill(8)
        #assert len(binary)==8,len(binary)
        bytes.append(binary)
    bits = []
    for byte in bytes:
        for bit in byte:
            bits.append(int(bit))
    return bits

print(translate(heart))
print(heart.encode('utf-16-be'))
print(heart.encode('utf-8'))

