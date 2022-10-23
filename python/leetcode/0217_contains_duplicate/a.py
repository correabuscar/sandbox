#!/usr/bin/python3

#https://leetcode.com/problems/contains-duplicate/

#least code version.

def containsDuplicate(nums: list[int]) -> bool:
        thisset = set(nums)
        if len(thisset) != len(nums):
            return True
        return False

nums = [1,2,3,1]
assert True == containsDuplicate(nums)
#assert False
nums = [1,2,3,4]
assert False == containsDuplicate(nums)
nums = [1,1,1,3,3,4,3,2,4,2]
assert True == containsDuplicate(nums)

