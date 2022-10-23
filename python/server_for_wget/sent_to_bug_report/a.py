#!/usr/bin/env python3

# test with: pushd /tmp; wget --tries=3 --timeout=2 --passive-ftp -O aha 127.1.2.9:8000/a.txt;cat aha;ls -la aha;echo 'Should be 14 bytes when no bug, or 20 if wget bug';popd
# or test with: $ ./go
# or test with: $ ./tst
#NOTE to self: to recompile 'wget' after changes, on Gentoo, after an initial: # ebuild `equery w wget` clean prepare
#do this for recompilations: # rm /var/tmp/portage/net-misc/wget-1.21.3-r1/.compiled; time ebuild `equery w wget` compile; chmod g+rx /var/tmp/portage/net-misc/wget-1.21.3-r1;chmod g+rx /var/tmp/portage/net-misc/wget-1.21.3-r1/work
#then for testing ur changes: $ ./tst


# XXX: user setable: (or pass 'bug' arg on cmdline)
# see_bug:bool=True
see_bug: bool = True
# XXX: set to True to see that wget no longer sends the Range header after a 302 redirect thus the entire file is appended to whatever was gotten thus far before the timeout occurred!
# XXX: set to False to see how wget should act if it had no bug! the 302 is avoided after the timeout, thus Range header is kept by wget

# some code from: https://stackoverflow.com/questions/2506932/how-do-i-redirect-a-request-to-a-different-url-in-python/47084250#47084250
# some code from: https://stackoverflow.com/questions/39350300/run-multiple-servers-in-python-at-same-time-threading/39350834#39350834
PORT1 = 8000
HOST1 = "127.0.2.9"

HOST2 = "127.0.3.12"
PORT2 = 8001

#nothing to mod below:
import re
import time
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
import sys

#import SimpleHTTPServer


done1: bool = False
done2: bool = False

# if len(sys.argv)-1 != 2:
#    print("""
# Usage: {} <port_number> <url>
#    """.format(sys.argv[0]))
#    sys.exit()

# FIXME: indentation!


class AddonBasic(
        BaseHTTPRequestHandler):
    # SimpleHTTPServer.SimpleHTTPRequestHandler):
    first_time: bool = True
    # note: \r\n becomes 2 chars: "^M" aka \r\n on the wire, or after utf8 conversion! so 14 bytes long!
    file: str = "Hello World.\r\n"
    file_utf8: bytes = file.encode("utf-8")
    file_size: int = len(file_utf8)  # 14  #hardcoded from "Hello World!\n"
    # this many bytes of file_utf8 to send when force_issuing_timeout==True  #XXX: this is an offset tho, it just happens to coincide with the length of the first how many bytes to send.
    upto_and_including: int = 6

    # def h200(self):
    def sendfile(self, force_issuing_timeout: bool = False):
        self.range_from, self.range_to = self._get_range_header()
        if self.range_from is None:
            code = 200
        else:
            #print("[Server] Sending the rest")
            code = 206
        self.send_response(code)
        self.send_header('Content-type', 'text/plain; charset=utf-8')
        # self.send_header('Content-Length',str(length))
        # some code from: https://gist.github.com/devgianlu/018b299f8817bf92350bf7bf70214e4d#file-serve_http-py-L126-L135
        if self.range_from is not None:
            if self.range_to is None or self.range_to >= self.file_size:
                self.range_to = self.file_size-1
            self.send_header("Content-Range",
                             "bytes %d-%d/%d" % (self.range_from,
                                                 self.range_to,
                                                 self.file_size))
            # Add 1 because ranges are inclusive
            cl = (1 + self.range_to - self.range_from)
        else:
            cl = self.file_size
        self.send_header("Content-Length", str(cl))
        self.end_headers()

        # print(self.headers)
        # if (self.headers["Range"]) == "bytes=6-":
        # else:
        if self.range_from is None:
            print("[Server] Sending full file")
            # self.wfile.write("Hello".encode('utf-8'))
            # self.wfile.write(" ".encode('utf-8')) #nvm//you've to resend this last byte when 206, wait, no you don't! it didn't send the '\n' and that's why
            if force_issuing_timeout:
                self.wfile.write(self.file_utf8[0:self.upto_and_including])
                print("Pretending to timeout")
                # this assumes you're passing wget arg --timeout=2
                time.sleep(2.1)
                return
            else:
                self.wfile.write(self.file_utf8)
                global done1
                done1 = True
        else:
            print(f"[Server] Sending the rest from {self.range_from}")
            self.wfile.write(self.file_utf8[self.range_from:])
        # self.wfile.write("World".encode('utf-8'))
        # self.wfile.write("!".encode('utf-8'))
        # self.wfile.write("\r\n".encode('utf-8')) #\r\n sends ^M and \n sends nothing!
        # if self.range_from is None:
        #    self.range_from=0 #doh
        # print(".....................",self.range_from,"->",self.file_size)
        #tf=self.file_utf8[self.range_from : self.file_size]
        # print(tf)
        # self.wfile.write(tf) #ahnvmFIXME: why does this yield "Hello Hello Wo" instead of "Hello World"
        # if self.range_from is None:
        #    self.wfile.write(self.file_utf8[self.upto_and_including+1:(self.file_size-self.upto_and_including)])
        # else:
        #    tf=self.file_utf8[self.range_from : (self.file_size - self.range_from)]

        # print(tf)

    def redir_to(self, host, port):
        self.send_response(302)
        self.send_header('Location', f"http://{host}:{port}{self.path}")
        self.end_headers()

    def _get_range_header(self):
        """ Returns request Range start and end if specified.
        If Range header is not specified returns (None, None)
        src: https://gist.github.com/devgianlu/018b299f8817bf92350bf7bf70214e4d#file-serve_http-py-L200
        """
        range_header = self.headers["Range"]
        if range_header is None:
            return (None, None)
        if not range_header.startswith("bytes="):
            print("Not implemented: parsing header Range: %s" % range_header)
            return (None, None)
        regex = re.compile(r"^bytes=(\d+)\-(\d+)?")
        rangething = regex.search(range_header)
        if rangething:
            from_val = int(rangething.group(1))
            if rangething.group(2) is not None:
                return (from_val, int(rangething.group(2)))
            else:
                return (from_val, None)
        else:
            print('CANNOT PARSE RANGE HEADER:', range_header)
            return (None, None)


class H1(AddonBasic):

    def do_GET(self):
        # print(self.headers)
        if (H1.first_time):
            #print("H1 first")
            H1.first_time = False
            self.redir_to(HOST2, PORT2)
            global see_bug, done1
            if not see_bug:
                done1 = True
        else:
            #print("H1 second+")
            self.sendfile()


class H2(AddonBasic):
    def do_GET(self):
        # print(self.headers)
        if (H2.first_time):
            #print("H2 first")
            H2.first_time = False
            self.sendfile(force_issuing_timeout=True)
        else:
            #print("H2 second+")
            global see_bug
            if see_bug:
                self.redir_to(HOST1, PORT1)  # enable this to cause wget bug
            # or allow the following to see how it should look like if no bug:
            else:
                self.sendfile()
            global done2
            done2 = True


httpd1 = HTTPServer((HOST1, PORT1), H1)
httpd2 = HTTPServer((HOST2, PORT2), H2)


def run1():
    while not done1:
        # print("B1")
        httpd1.handle_request()  # blocking
        # print("A1")
    #print("1 is done")


def run2():
    while not done2:
        # print("B2")
        httpd2.handle_request()  # blocking
        # print("A2")
    #print("2 is done")


if __name__ == '__main__':
    #print(len(sys.argv))
    if len(sys.argv)-1 >= 1:
        if sys.argv[1] == "bug":
            print("Bugged version on")
            see_bug = True
        else:
            print("unbugged version on")
            see_bug = False
    t1 = threading.Thread(target=run1)
    t2 = threading.Thread(target=run2)
    t1.start()
    t2.start()
#    print("Will wait max 10 seconds before autokilling server threads...")
#    while (not (done1 or done2)):
#        time.sleep(0.1)
#    print("Cleaning up server threads...")
#    httpd1.shutdown()
#    httpd2.shutdown()
#    print("Waiting for shutdowns...")
    t1.join(1)
    # t2.close()
    t2.join(1)

