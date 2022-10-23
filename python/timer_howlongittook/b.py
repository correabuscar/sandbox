#!/usr/bin/python3



import time as t
def TimeTakenDecorator(func):
    #src: https://stackoverflow.com/a/70954147/19999437
    def wraper(*args,**kwargs):
        start = t.time()
        func(*args,**kwargs)
        end = t.time()
        print('Elapsed time: ', end - start)
    return wraper

#@TimeTakenDecorator
#def gulp1():
#    a_set = set()
#    something=12345
#    #...
#    pre_len = len(a_set)
#    a_set.add(something)
#    if pre_len != len(a_set):
#        print(f"The element({something}) was added therefore it was not already in the set.")
#    else:
#        print(f"The element({something}) was not added because it was already in the set.")
#
#    pre_len = len(a_set)
#    a_set.add(something)
#    if pre_len != len(a_set):
#        print(f"The element({something}) was added therefore it was not already in the set.")
#    else:
#        print(f"The element({something}) was not added because it was already in the set.")

something=5_234_567
REPEATS=10_000_000

@TimeTakenDecorator
def re_set():
    print("Initializing the set:")
    global g2_set
    g2_set = set()
    for i in range(10_000_000):
        g2_set.add(i)

@TimeTakenDecorator
def slower():
    print("Using double lookup:")
    for i in range(REPEATS):
        double_lookup_add()

@TimeTakenDecorator
def faster():
    print("Using len():")
    for i in range(REPEATS):
        no_double_lookup_add()

def double_lookup_add():
    global g2_set
    if something not in g2_set:
        g2_set.add(something)
        pass
        #print(f"The element({something}) was added therefore it was not already in the set.")
    else:
        pass
        #print(f"The element({something}) was not added because it was already in the set.")

def no_double_lookup_add():
    global g2_set
    pre_len = len(g2_set)
    g2_set.add(something)
    if pre_len != len(g2_set):
        pass
        #print(f"The element({something}) was added therefore it was not already in the set.")
    else:
        pass
        #print(f"The element({something}) was not added because it was already in the set.")


def main():
    #gulp1()
    re_set()
    faster()
    re_set()
    slower()
    print("Once more in reverse order:")
    re_set()
    slower()
    re_set()
    faster()

main();
