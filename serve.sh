#!/bin/bash

set -ev

. ./build.sh

cargo run --release --bin server
