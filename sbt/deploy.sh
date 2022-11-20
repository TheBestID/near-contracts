#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy sbt.soul_dev.testnet ./target/wasm32-unknown-unknown/release/contract.wasm