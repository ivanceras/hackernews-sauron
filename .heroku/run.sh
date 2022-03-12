#
# Execute shell script for building the project in heroku
# Set your project buildpacks first to:
# ```
# heroku buildpacks:set https://github.com/niteoweb/heroku-buildpack-shell.git
# ```
#
set -e

# Install rust compilter via rustup

curl https://sh.rustup.rs -sSf | sh -s --  --profile minimal --default-toolchain stable -y

# set the enviornment variable
source $HOME/.cargo/env

# Install wasm-pack
cargo install wasm-pack

# build the client project
wasm-pack build client  --target web --release

# clean updater the client has build to save some space as heroku has a soft limit of 300MB
cargo clean

# build the server project
cargo build -p server  --release
