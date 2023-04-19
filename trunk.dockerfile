FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

RUN cargo install trunk wasm-bindgen-cli

FROM builder-base as runner

ENTRYPOINT [ "trunk" ]
