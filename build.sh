#!/bin/bash
set -ev

cd "$(dirname "$0")"

wasm-pack build client --release --target web

cargo build --release --bin server
