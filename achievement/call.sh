#!/bin/sh

echo ">> Calling contract"

near call achievements.soul_dev.testnet mint '{"_achievement_data": {"achievement_id": 1, "achievement_type": 1, "issuer": 1, "owner": 1, "is_accepted": false, "verifier": 1, "is_verified": false, "data_address": "data", "balance": 1}}' --accountId andzhi.testnet --deposit 2
near create-account achievements.soul_dev.testnet --initialBalance 10 --masterAccount soul_dev.testnet