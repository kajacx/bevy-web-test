#!/usr/bin/sh

cd plugin && cargo build --target wasm32-unknown-unknown && cd ..  && \
cd host-wasmer && \
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner cargo run --target wasm32-unknown-unknown
