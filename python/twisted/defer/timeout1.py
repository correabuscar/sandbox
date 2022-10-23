#!/bin/python3

#src: https://twistedmatrix.com/documents/current/core/howto/defer.html#timeouts
import random
from twisted.internet import task #type: ignore

def f():
    return "Hopefully this will be called in 3 seconds or less"

def main(reactor):
    delay = random.uniform(1, 5)

    def called(result):
        print("{0} seconds later:".format(delay), result)

    d = task.deferLater(reactor, delay, f)
    d.addTimeout(3, reactor).addBoth(called)

    return d

# f() will be timed out if the random delay is greater than 3 seconds
task.react(main)
