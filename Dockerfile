# using 2 build step
FROM rust:1.60.0-slim-bullseye as build

RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config
RUN cargo install wasm-pack
COPY . .
RUN ./build.sh


# The actual server image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates
EXPOSE 3030
COPY --from=build ./target/release/server /usr/bin/server
CMD /usr/bin/server

