#!/usr/bin/python3

#https://leetcode.com/problems/contains-duplicate/

#least cpu/mem version.

def containsDuplicate(nums: list[int]) -> bool:
    thisset: set[int] = set()
    for num in nums:
        #XXX: damn, set.add() returns None always, instead of bool! bye bye performance, double lookup it is then.
        if num not in thisset:
            thisset.add(num)
        else:
            return True
        #OR, better yet, single lookup:
        #XXX: actually this is slower! by 9.4% or 51% as per my (possibly wrong?) tests here: https://stackoverflow.com/a/73727392/19999437
        #pre=len(thisset)
        #thisset.add(num)
        #if pre == len(thisset):
        #    return True
    return False

nums = [1,2,3,1]
assert True == containsDuplicate(nums)
#assert False
nums = [1,2,3,4]
assert False == containsDuplicate(nums)
nums = [1,1,1,3,3,4,3,2,4,2]
assert True == containsDuplicate(nums)

