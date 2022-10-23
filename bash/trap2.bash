#!/bin/bash

#XXX: ok so first C-c will exit cleanup() and re-run it(unless 'trap - ...' is first!), any subsequent C-c will just exit current command(eg. sleep), but cleanup() will continue to the next command from cleanup() until cleanup is completed!

nothing(){
  echo "ignoring C-c but wtw current command was(eg. sleep) was interrupted!"
}

cleanup() {
  #trap - EXIT SIGINT #unless this is first!
  trap nothing SIGINT #you can keep this for debugging, but the same effect is kept as mentioned in XXX above! with the exception that cleanup isn't re-run on first C-c!!!
  #trap - SIGINT # then first C-c interrupts entirely!
  echo "sleeping for 5"
  sleep 5 #C-c here! it will re-run cleanup! but next C-c will just C-c sleep AND continue to 'done sleeping'!
  echo "done sleeping, doing another 5 tho"
  sleep 5
  trap - EXIT SIGINT
  echo "cleanup exit"
  exit 0
}

trap cleanup EXIT SIGINT
