#!/usr/bin/python3

import hashlib

m = hashlib.sha256()
m.update(b"1")
print(m.digest())
print(m.digest_size)
print(m.block_size)
print(m.hexdigest())
m.update(b"2")
print(m.hexdigest())

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


#Looking for something like this:
r = hashlib.sha256rev("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069")
r.update(b"!dlroW")
r.update(b" olleH")
r2 = hashlib.sha256rev("7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069")
r2.update(b"!dlroW olleH")
print(r.hexdigest())  #e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
assert r.hexdigest() == r2.hexdigest()

#for: https://stackoverflow.com/questions/73787368/how-can-the-sha256-hash-be-checked-to-match-against-the-reverse-input-bytes-in


