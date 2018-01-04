#!/bin/bash

#export RUST_LOG=warn
#^ don't need this during compiling! would only need it on run (no tests use it currently either)

set -ex
echo '!! Building...'
time cargo build --verbose --all
echo '!! Testing...'
time cargo test --verbose --all
