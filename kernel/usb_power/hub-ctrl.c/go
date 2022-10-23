#!/bin/bash

#shellcheck disable=SC2046
gcc hub-ctrl.c -o hub-ctrl -D_FORTIFY_SOURCE=2 -O2 -ggdb $(pkgconf --print-errors  --cflags --libs libusb)
gcc hub-ctrl_orig.c -o hub-ctrl0 -D_FORTIFY_SOURCE=2 -O2 -ggdb $(pkgconf --print-errors  --cflags --libs libusb)

