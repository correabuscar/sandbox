#!/bin/bash

set -ex
echo '!! Building...'
cargo build --verbose --all
echo '!! Testing...'
cargo test --verbose --all
