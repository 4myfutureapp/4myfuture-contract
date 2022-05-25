#!/bin/bash
echo 'building contract...'
cargo build --target wasm32-unknown-unknown --release && mkdir -p ./out && cp target/wasm32-unknown-unknown/release/formyfuture.wasm ./out/formyfuture.wasm
echo
echo 'deploying contract...'
near dev-deploy -f ./out/formyfuture.wasm

