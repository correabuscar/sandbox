#!/usr/bin/python3

#https://leetcode.com/problems/palindrome-linked-list/

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        #not my code, #src: https://leetcode.com/problems/palindrome-linked-list/discuss/2593397/python-90
        #_l = []
        #while head:
        #    _l.append(head.val)
        #    head=head.next
        #return _l == _l[::-1] #so reversed == normal

        #TODO: my own version

        #find middle of linked list? slow&fast pointers then.
        #1 ,2,3,4,5,6
        #sf,
        #  ,s,f
        #  , ,s, ,f

        #2, 4,3,4,2 = palindrome

        #1, 2,3,4,5
        #sf,
        #  ,s,f,
        #  , ,s, ,f
        #then you reverse from s.next onwards. or, consider the second half as beginning from there (which will be last element after reversal)
        #1 ,2,3,4
        #sf,
        #  ,s,f,
        #1, 2,3
        #sf,
        #   s,f
        #1 ,2
        #sf,
        #1,
        #sf,
        #there's always one element, "Constraints:" says!


        pass


ll=ListNode(1,ListNode(2,ListNode(2,ListNode(1))))

c=Solution()
assert True == c.isPalindrome(ll)
ll=ListNode(3,ListNode(1,ListNode(2,ListNode(2,ListNode(1,ListNode(3))))))
assert True == c.isPalindrome(ll)
ll=ListNode(1,ListNode(2))
assert False == c.isPalindrome(ll)
ll=ListNode(1,ListNode(2,ListNode(1)))
assert True == c.isPalindrome(ll)
ll=ListNode(3,ListNode(1,ListNode(4,ListNode(1,ListNode(3)))))
assert True == c.isPalindrome(ll)

#1,2,   2,1
#1,3,2, 2,3,1
#3,1,2, 2,1,3

