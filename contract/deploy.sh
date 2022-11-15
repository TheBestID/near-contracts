#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy contract.andzhi.testnet ./target/wasm32-unknown-unknown/release/contract.wasm