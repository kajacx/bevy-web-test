#!/usr/bin/sh

cd plugin && cargo build --target wasm32-unknown-unknown && cd ..  && \
cd wasm-only && \
wasm-pack build --target web && \
# wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/bevy-web-test.wasm && \
cd pkg && docker-compose down && docker-compose up -d && cd .. && \
cd .. && \
echo "Done, view at http://localhost:8091"
