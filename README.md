# Hackernew Clone

A hacker news clone in ~1k lines of rust.
This is using [sauron](https://github.com/ivanceras/sauron) web-framework.


## Feature
- [X] Isomorphic
    - [X] Completely identical server-side rendered and client-side rendered
    - [X] No weird font jumping.
- [X] Resilient
    - [X] Can work without javascript enabled.
    - [X] Can work without the page server*.
        - You can kill the server after the initial serve.
    - *Note: This will not work if both points of failure are encountered at the same time.

## Quickstart

Prerequisite:

```sh
cargo install wasm-pack
```

Compile and run
```sh
git clone --depth=1 https://github.com/ivanceras/hackernews-sauron

cd hackernews-sauron

wasm-pack build client --release --target web

cargo run --release --bin server
```

Navigate to http://localhost:3030


