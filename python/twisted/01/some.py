#!/usr/bin/pypy3 -bb

#src: https://stackoverflow.com/questions/39127928/twisted-autobahn-websocket-being-initialized-twice-with-wss/39134379#39134379

from datetime import datetime
from autobahn.twisted.websocket import WebSocketServerProtocol, WebSocketServerFactory
#FIXME: never ran due to:     ModuleNotFoundError: No module named 'autobahn'

from twisted.internet.ssl import CertificateOptions, PrivateCertificate, Certificate, KeyPair
from twisted.internet.endpoints import SSL4ServerEndpoint
from twisted.internet.task import react
from OpenSSL import crypto


CERT_KEY = "certificate.key"
CERT_PATH = "certificate.crt"


def log(msg):
    print("{}: {}".format(str(datetime.now()), msg))


class TestProtocol(WebSocketServerProtocol):
    def __init__(self):
        super(TestProtocol, self).__init__()
        log("Test protocol init")

    def connectionLost(self, reason):
        WebSocketServerProtocol.connectionLost(self, reason)
        log("Connection closed: Reason is {}".format(reason))



class TestProtocolFactory(WebSocketServerFactory):
    protocol = TestProtocol


def init_websocket_protocol(reactor, port):
    with open(CERT_KEY) as key_file, open(CERT_PATH) as cert_file:
        key = KeyPair.load(key_file.read(), crypto.FILETYPE_PEM).original
        cert = Certificate.loadPEM(cert_file.read()).original
    ctx = CertificateOptions(
        privateKey=key,
        certificate=cert,
    )
    return SSL4ServerEndpoint(reactor, port, ctx)


def main(reactor):
    ep = init_websocket_protocol(reactor, 9000)
    ep.listen(TestProtocolFactory())
    reactor.run()


if __name__ == '__main__':
    react(main)
