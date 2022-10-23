#!/bin/python

#src: https://twistedmatrix.com/documents/current/core/howto/defer.html

def synchronousIsValidUser(user):
    '''
    Return true if user is a valid user, false otherwise
    '''
    #print("in sync")
    return user in ["Alice", "Angus", "Agnes"]

from twisted.internet import reactor, defer #type: ignore

def asynchronousIsValidUser(user):
    d = defer.Deferred()
    #reactor.callLater(2, d.callback, user in ["Alice", "Angus", "Agnes"])
    reactor.callLater(2, d.callback, synchronousIsValidUser(user))
    print("in async")
    return d

from twisted.internet import defer

def printResult(result):
    if result:
        print(f"User is authenticated {type(result)}")
    else:
        print(f"User is not authenticated {type(result)}")

def authenticateUser(isValidUser, user):
    d = defer.maybeDeferred(isValidUser, user)
    d.addCallback(printResult)


d=authenticateUser(asynchronousIsValidUser, 'blah')
authenticateUser(synchronousIsValidUser, 'blah')
reactor.callLater(4, reactor.stop)
reactor.run()
