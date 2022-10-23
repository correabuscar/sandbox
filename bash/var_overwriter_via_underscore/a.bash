#!/bin/bash

echo $SECONDS_____$RANDOM____$SECONDS
#^ the last $SECONDS overwrites all of them
#ie. result is 0
#OK, it's because _ is considered part of the name!
