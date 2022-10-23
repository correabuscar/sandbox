#!/usr/bin/python

#!/usr/bin/pypy3 -bb 
#^ can't use that due to: ModuleNotFoundError: No module named 'twisted'


#src: https://twistedmatrix.com/documents/current/core/howto/defer.html
from twisted.internet import reactor, defer # type: ignore
import time

def getDummyData(inputData):
    """
    This function is a dummy which simulates a delayed result and
    returns a Deferred which will fire with that result. Don't try too
    hard to understand this.
    """
    print('getDummyData called')
    deferred = defer.Deferred()
    # simulate a delayed result by asking the reactor to fire the
    # Deferred in 2 seconds time with the result inputData * 3
    reactor.callLater(2, deferred.callback, inputData * 3)
    return deferred

def cbPrintData(result):
    """
    Data handling function to be added as a callback: handles the
    data by printing the result
    """
    print('Result received: {}'.format(result))

deferred = getDummyData(3)
#time.sleep(3) #so, lookslike it doesn't matter that I add the callback later than the result being available! oh, that's because the reactor isn't started, meh; so the 2 sec later call from getDummyData isn't happening until after reactor.run()
deferred.addCallback(cbPrintData)

# manually set up the end of the process by asking the reactor to
# stop itself in 4 seconds time
reactor.callLater(4, reactor.stop)
# start up the Twisted reactor (event loop handler) manually
print('Starting the reactor')
reactor.run()
