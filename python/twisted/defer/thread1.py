#!/bin/python3

#src: https://twistedmatrix.com/documents/current/core/howto/gendefer.html#integrating-blocking-code-with-twisted

def largeFibonnaciNumber():
    """
    Represent a long running blocking function by calculating
    the TARGETth Fibonnaci number
    """
    TARGET = 10000

    first = 0
    second = 1

    for i in range(TARGET - 1):
        new = first + second
        first = second
        second = new

    return second

from twisted.internet import threads, reactor # type: ignore

def fibonacciCallback(result):
    """
    Callback which manages the largeFibonnaciNumber result by
    printing it out
    """
    print("largeFibonnaciNumber result =", result)
    # make sure the reactor stops after the callback chain finishes,
    # just so that this example terminates
    reactor.stop()

def run():
    """
    Run a series of operations, deferring the largeFibonnaciNumber
    operation to a thread and performing some other operations after
    adding the callback
    """
    # get our Deferred which will be called with the largeFibonnaciNumber result
    d = threads.deferToThread(largeFibonnaciNumber)
    # add our callback to print it out
    d.addCallback(fibonacciCallback)
    print("1st line after the addition of the callback")
    print("2nd line after the addition of the callback")

if __name__ == '__main__':
    run()
    reactor.run()

