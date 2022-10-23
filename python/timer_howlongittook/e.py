#!/usr/bin/python3

import time

class A_Len_Test(set):
    def __len__(self):
        #time.sleep(1)
        return super().__len__()

a=A_Len_Test()

print("First:")
print(len(a))
print("Second:")
print(len(a))
