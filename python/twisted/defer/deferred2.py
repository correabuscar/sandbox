#!/usr/bin/python3

#src: https://twistedmatrix.com/documents/current/core/howto/defer.html

from twisted.internet import reactor, defer # type: ignore
from twisted.python.failure import Failure #type: ignore

class Getter:
    def gotResults(self, x):
        """
        The Deferred mechanism provides a mechanism to signal error
        conditions.  In this case, odd numbers are bad.

        This function demonstrates a more complex way of starting
        the callback chain by checking for expected results and
        choosing whether to fire the callback or errback chain
        """
        if self.d is None:
            print("Nowhere to put results")
            return

        d = self.d
        self.d = None
        if x % 2 == 0:
            d.callback(x*3)
            #why not just:
            return x*3
        else:
            #d.errback(ValueError("You used an odd number!"))
            #why not just:
            raise ValueError("You used an odd number!")
            #^ yep that's much better! because now I get a stacktrace too!

    def _toHTML(self, r):
        """
        This function converts r to HTML.

        It is added to the callback chain by getDummyData in
        order to demonstrate how a callback passes its own result
        to the next callback
        """
        return "Result: %s" % r

    def getDummyData(self, x):
        """
        The Deferred mechanism allows for chained callbacks.
        In this example, the output of gotResults is first
        passed through _toHTML on its way to printData.

        Again this function is a dummy, simulating a delayed result
        using callLater, rather than using a real asynchronous
        setup.
        """
        self.d = defer.Deferred()
        # simulate a delayed result by asking the reactor to schedule
        # gotResults in 2 seconds time
        reactor.callLater(2, self.gotResults, x)
        self.d.addCallback(self._toHTML)
        return self.d

def cbPrintData(result):
    print(result)

def ebPrintError(failure):
    import sys
    sys.stderr.write(str(failure))
    #raise RuntimeError #ok this is good for stacktrace
    #return Failure(failure) #no stacktrace
    return failure #no stacktrace, unless ofc already raised in prev. call!
    #or no error if non-Failure or nothing is returned ie. "Note: If an errback doesn’t return anything, then it effectively returns None, meaning that callbacks will continue to be executed after this errback." src: https://twistedmatrix.com/documents/current/core/howto/defer.html

# this series of callbacks and errbacks will print an error message
g = Getter()
d = g.getDummyData(3)
d.addCallback(cbPrintData)
d.addErrback(ebPrintError)

# this series of callbacks and errbacks will print "Result: 12"
g = Getter()
d = g.getDummyData(4)
d.addCallback(cbPrintData)
d.addErrback(ebPrintError)

reactor.callLater(4, reactor.stop)
reactor.run()
