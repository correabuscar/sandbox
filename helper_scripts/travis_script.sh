#!/bin/bash

set -e
echo '!! Building...'
cargo build --verbose --all
echo '!! Testing...'
cargo test --verbose --all
