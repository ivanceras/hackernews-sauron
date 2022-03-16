#!/bin/bash

set -ev

# when called from other directory, it will always cd first into this directory
cd "$(dirname "$0")"

. ./build.sh

cargo run --release --bin server --features use-port-80
