#!/bin/python

#src: https://docs.python.org/3/howto/curses.html
import curses
from curses import wrapper
import time

def main(stdscr):
    # Clear screen
    stdscr.clear()

    pad = curses.newpad(100, 100)
    # These loops fill the pad with letters; addch() is
    # explained in the next section
    for y in range(0, 99):
        for x in range(0, 99):
            pad.addch(y,x, ord('a') + (x*x+y*y) % 26)

    # Displays a section of the pad in the middle of the screen.
    # (0,0) : coordinate of upper-left corner of pad area to display.
    # (5,5) : coordinate of upper-left corner of window area to be filled
    #         with pad content.
    # (20, 75) : coordinate of lower-right corner of window area to be
    #          : filled with pad content.
    pad.refresh( 0,0, 5,5, 20,75)

    time.sleep(5)   # Delays for 5 seconds so you can C-c for tests

    # This raises ZeroDivisionError when i == 10.
    for i in range(0, 11):
        v = i-10
        stdscr.addstr(i, 0, '10 divided by {} is {}'.format(v, 10/v))

    stdscr.refresh()
    stdscr.getkey()

wrapper(main)
