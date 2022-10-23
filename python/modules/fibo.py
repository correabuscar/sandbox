#!/usr/bin/pypy3 -bb
#src: https://docs.python.org/3/tutorial/modules.html

# Fibonacci numbers module

def fib(n):    # write Fibonacci series up to n
    a, b = 0, 1
    while a < n:
        print(a, end=' ')
        a, b = b, a+b
    print()

def fib2(n):   # return Fibonacci series up to n
    result = []
    a, b = 0, 1
    while a < n:
        result.append(a)
        a, b = b, a+b
    return result

#Within a module, the module’s name (as a string) is available as the value of the global variable __name__
if __name__ == "__main__":
    import os
    full=os.path.realpath(__file__)
    justfname=os.path.basename(full)
    modulename=os.path.splitext(justfname)[0]
    #print(os.path.splitext(full))
    print(f"You attempted to execute python module {justfname} (aka {full}) directly, use 'import {modulename}' instead.")
else:
    print("hello from import-time of module '" + __name__ + "'")


