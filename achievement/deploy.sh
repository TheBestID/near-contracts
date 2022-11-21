#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy achievements.soul_dev.testnet ./target/wasm32-unknown-unknown/release/contract.wasm