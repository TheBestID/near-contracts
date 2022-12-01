#!/bin/sh

echo ">> Calling contract"

near call sbt.soul_dev.testnet mint '{"new_id": 1, "account": "andzhi.testnet"}' --accountId sbt.soul_dev.testnet
near call sbt.soul_dev.testnet claim '{"new_git_hash": "0xcccccccccccccccccccccccccccccc", "new_email_hash": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}' --accountId andzhi.testnet --gas=75000000000000
near call sbt.soul_dev.testnet burn '{}' --accountId andzhi.testnet --gas=75000000000000
near call sbt.soul_dev.testnet get_account_id '{"user_id": 1}' --accountId sbt.andzhi.testnet 
near call sbt.soul_dev.testnet get_user_id '{"account": "andzhi.testnet"}' --accountId sbt.andzhi.testnet
near call sbt.soul_dev.testnet has_soul '{"account" : "andzhi.testnet"}' --accountId andzhi.testnet
near call sbt.soul_dev.testnet get_hashed_data '{}' --accountId andzhi.testnet
near call sbt.soul_dev.testnet soul_is_minted_not_claimed '{"account": "andzhi.testnet"}' --accountId sbt.andzhi.testnet
near create-account sbt.soul_dev.testnet --initialBalance 5 --masterAccount andzhi.testnet