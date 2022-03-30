#!/bin/sh

echo Building package $1
cargo build --target wasm32-unknown-unknown --release --package $1

echo Optimising wasm
ic-cdk-optimizer target/wasm32-unknown-unknown/release/$1.wasm -o target/wasm32-unknown-unknown/release/$1-opt.wasm

echo Copy bucket wasm to assets
cp target/wasm32-unknown-unknown/release/$1-opt.wasm src/client_web/assets/$1-opt.wasm