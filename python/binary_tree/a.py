#!/usr/bin/python3
#made to see if: is contains() bugged?(but it's obviously not! == was handled first!) in https://www.youtube.com/watch?v=oSWTXtMglKE
from __future__ import annotations
#^needed! for self-referencing from: https://stackoverflow.com/a/69440811/19999437
#otherwise: NameError: name 'Node' is not defined. Did you mean: 'None'?  at `left: Node[T]` below!

#generics via https://medium.com/@steveYeah/using-generics-in-python-99010e5056eb
from typing import TypeVar, Generic
T=TypeVar("T", int, str) #FIXME: ensure it's any type that can be compared?
#^ also https://docs.python.org/3/library/typing.html

#I have to use Generic[T] as base class, according to 'mypy' which does the checking(from nvim/neovim/neovide) or else:
#a.py:14: error: Type variable "a.T" is unbound
#a.py:14: note: (Hint: Use "Generic[T]" or "Protocol[T]" base class to bind "T" inside a class)
#a.py:14: note: (Hint: Use "T" in function signature to bind "T" inside a function)
#

class Node(Generic[T]):
    left: Node[T]
    right: Node[T]
    data: T
    def __init__(self, data: T):
        self.data=data
        left,right=None, None


    def insert(self, value: T) -> None:
        if (value <= self.data):
            if (self.left is None):
                self.left=Node(value)
            else:
                self.left.insert(value)
        else:
            if self.right is None:
                right=Node(value)
            else:
                self.right.insert(value)

    def contains(self, value: T) -> bool:
        if (value == self.data):
            return True
        elif (value < self.data):
            if (self.left is None):
                return False
            else:
                return self.left.contains(value)
        else:
            if (self.right is None):
                return False
            else:
                return self.right.contains(value)


