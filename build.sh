
#!/bin/bash
set -ev

wasm-pack build client --release --target web
