#!/bin/sh

echo ">> Calling contract"

near call achievements.soul_dev.testnet mint '{"_achievement_data": {"achievement_id": 1, "achievement_type": 1, "issuer": 1, "owner": 1, "is_accepted": false, "verifier": 1, "is_verified": false, "data_address": "data", "balance": 1}}' --accountId andzhi.testnet --deposit 2
near call achievements.soul_dev.testnet accept_achievement '{"_achievement_id": 1}' --accountId andzhi.testnet
near call achievements.soul_dev.testnet burn '{"_achievement_id": 1}' --accountId andzhi.testnet
near call achievements.soul_dev.testnet verify_achievement '{"_achievement_id": 1}' --accountId andzhi.testnet
near call achievements.soul_dev.testnet get_achievement_data '{"_achievement_id": 1}' --accountId andzhi.testnet
near create-account achievements.soul_dev.testnet --initialBalance 10 --masterAccount soul_dev.testnet