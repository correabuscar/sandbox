#!/bin/python3

#src: forgot, from klein docs somewhere

from klein import Klein # type: ignore
app = Klein()

from twisted.internet import endpoints, reactor # type: ignore
from twisted.web.server import Site # type: ignore

# Create desired endpoint
endpoint_description = "tcp:port=8080:interface=127.0.0.2"
endpoint = endpoints.serverFromString(reactor, endpoint_description)

# This actually starts listening on the endpoint with the Klein app
endpoint.listen(Site(app.resource()))

# After doing other things like setting up logging,
# starting other services in the reactor or
# listening on other ports or sockets:
if __name__ == '__main__':
    reactor.run()

