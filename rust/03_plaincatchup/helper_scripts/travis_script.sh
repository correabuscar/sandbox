#!/bin/bash

#export RUST_LOG=warn
#^ don't need this during compiling! would only need it on run (no tests use it currently either)

set -ex
time {
echo '!! Building...'
#time cargo build --verbose --all
#time cargo clippy --all-targets --all-features --verbose --all
time cargo clippy --all-targets --verbose --all
echo '!! Testing...'
time cargo test --verbose --all
}
