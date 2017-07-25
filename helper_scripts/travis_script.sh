#!/bin/bash

set -ex
echo '!! Building...'
time cargo build --verbose --all
echo '!! Testing...'
time cargo test --verbose --all
