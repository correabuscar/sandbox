#!/usr/bin/python3

#ignore this.

#fixed it like dis:
def mata2(n:int)->int:
    mask=1
    biti=1
    while (mask)<n:
        biti+=1
        mask*=2
        if (mask%2==0):
            mask+=1
        print(f"{biti} {mask}")
    print(biti)
    aw=len(bin(n))-2
    assert aw == biti, f"{aw}!={biti}"
    return biti

assert 1 == mata2(1)
assert 2 == mata2(2)
assert 2 == mata2(3)
assert 3 == mata2(4)
assert 3 == mata2(5)
assert 3 == mata2(6)
assert 3 == mata2(7)
assert 4 == mata2(8)

