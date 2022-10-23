#!/usr/bin/python3

#src: https://stackoverflow.com/a/70954147/19999437

import time as t
def TimeTakenDecorator(func):
    def wraper(*args,**kwargs):
        start = t.time()
        func(*args,**kwargs)
        end = t.time()
        print('Time taken for fun program: ', end - start)
    return wraper

@TimeTakenDecorator
def hello(s):
    for _ in range(1_000_000):
        pass
    print(s)

hello("test")
