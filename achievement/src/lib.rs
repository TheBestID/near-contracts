// SPDX-License-Identifier: MIT
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::collections::Vector;

use near_sdk::{env, log, near_bindgen, require, AccountId, Balance, Gas, Promise};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::Index;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Achievement {
    achievement_id: u128,
    achievement_type: u128,
    issuer: u128,
    owner: u128,
    is_accepted: bool,
    verifier: u128,
    is_verified: bool,
    data_address: Vector<u8>,
    balance: Balance,
}

#[derive(Deserialize, Serialize)]
pub struct IOAchievement {
    achievement_id: u128,
    achievement_type: u128,
    issuer: u128,
    owner: u128,
    is_accepted: bool,
    verifier: u128,
    is_verified: bool,
    data_address: String,
    balance: Balance,
}
// TODO: Maybe we should store balance as special type?

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AchievementToken {
    account_of_id: HashMap<u128, AccountId>,
    id_of_account: HashMap<AccountId, u128>,
    achievements: HashMap<u128, Achievement>,
    issuers_achievements: HashMap<u128, Vec<u128>>,
    owners_achievements: HashMap<u128, Vec<u128>>,
}

pub const XCC_GAS: Gas = Gas(20_000_000_000_000);
pub const NEAR: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
impl AchievementToken {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            achievements: HashMap::new(),
            account_of_id: HashMap::new(),
            id_of_account: HashMap::new(),
            issuers_achievements: HashMap::new(),
            owners_achievements: HashMap::new(),
        }
    }

    pub fn set_id_of_account(&mut self, _user: AccountId, _id: u128) {
        require!(
            env::predecessor_account_id() == "sbt.soul_dev.testnet".parse().unwrap(),
            format!(
                "Only SBT contract can call this function, but it is called by {}",
                env::signer_account_id()
            )
        );
        self.account_of_id.insert(_id.clone(), _user.clone());
        self.id_of_account.insert(_user.clone(), _id.clone());
        log!("Inserted {}, {} from Achievements", _id, _user);
    }

    pub fn remove_id_of_account(&mut self, _user: AccountId, _id: u128) {
        require!(
            env::predecessor_account_id() == "sbt.soul_dev.testnet".parse().unwrap(),
            format!(
                "Only SBT contract can call this function, but it is called by {}",
                env::signer_account_id()
            )
        );
        self.account_of_id.remove(&_id);
        self.id_of_account.remove(&_user);
        log!("Removed {}, {} from Achievements", _id, _user);
    }

    #[payable]
    pub fn mint(&mut self, _achievement_data: IOAchievement) {
        require!(
            !self
                .achievements
                .contains_key(&_achievement_data.achievement_id),
            "Achievement id already exists"
        );
        require!(
            self.id_of_account.contains_key(&env::signer_account_id())
                && self.id_of_account[&env::signer_account_id()] == _achievement_data.issuer,
            "Only you can be an issuer"
        );
        require!(
            env::attached_deposit() >= _achievement_data.balance * NEAR,
            "Not enough deposit attached"
        );
        require!(
            env::attached_deposit() >= u128::MAX / NEAR,
            format!(
                "You can't attach so much NEAR (attached {}), maximum is {}.",
                env::attached_deposit(),
                u128::MAX / NEAR
            )
        );
        let return_money = env::attached_deposit() - _achievement_data.balance * NEAR;
        log!("Returns {} NEAR to msg.sender", return_money / NEAR);
        Promise::new(env::signer_account_id()).transfer(return_money);

        let current_id = _achievement_data.achievement_id;
        let mut str_as_vector: Vector<u8> = Vector::new(b"m");
        for ch in _achievement_data.data_address.chars() {
            let x = ch as u8;
            str_as_vector.push(&x);
        }
        let current_achievement = Achievement {
            achievement_id: _achievement_data.achievement_id,
            achievement_type: _achievement_data.achievement_type,
            issuer: _achievement_data.issuer,
            owner: _achievement_data.owner,
            is_accepted: _achievement_data.is_accepted,
            verifier: _achievement_data.verifier,
            is_verified: _achievement_data.is_verified,
            data_address: str_as_vector,
            balance: _achievement_data.balance,
        };
        self.achievements.insert(current_id, current_achievement);

        if !self
            .issuers_achievements
            .contains_key(&self.achievements[&current_id].issuer)
        {
            self.issuers_achievements
                .insert(self.achievements[&current_id].issuer, Vec::new());
        }

        if !self
            .owners_achievements
            .contains_key(&self.achievements[&current_id].owner)
        {
            self.owners_achievements
                .insert(self.achievements[&current_id].owner, Vec::new());
        }

        self.issuers_achievements
            .get_mut(&self.achievements[&current_id].issuer)
            .unwrap()
            .push(current_id);
        self.owners_achievements
            .get_mut(&self.achievements[&current_id].owner)
            .unwrap()
            .push(current_id);
    }

    pub fn burn(&mut self, _achievement_id: u128) {
        let signer = env::signer_account_id();
        require!(
            self.id_of_account.contains_key(&signer),
            "No such account exists"
        );
        require!(
            self.achievements.contains_key(&_achievement_id),
            "Achievement must exist to burn it"
        );
        let signer_account_id = self.id_of_account[&signer];
        let current_issuer_id = self.achievements[&_achievement_id].issuer;
        require!(
            current_issuer_id == signer_account_id,
            "Only issuer can burn achievement"
        );
        for i in 0..self.issuers_achievements[&current_issuer_id].len() {
            if _achievement_id == self.issuers_achievements[&current_issuer_id][i] {
                self.issuers_achievements
                    .get_mut(&current_issuer_id)
                    .unwrap()
                    .remove(i);
                break;
            }
        }

        let current_owner_id = self.achievements[&_achievement_id].owner;
        for i in 0..self.owners_achievements[&current_owner_id].len() {
            if _achievement_id == self.owners_achievements[&current_owner_id][i] {
                self.owners_achievements
                    .get_mut(&current_owner_id)
                    .unwrap()
                    .remove(i);
                break;
            }
        }
        self.achievements.remove(&_achievement_id);
    }

    pub fn update_owner(&mut self, _achievement_id: u128, _new_owner: AccountId) {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        require!(
            self.achievements[&_achievement_id].owner == 0,
            "Owner of this achievement can not be changed"
        );
        require!(
            self.id_of_account.contains_key(&_new_owner),
            "Account of new owner doesn't exist"
        );
        let signer = env::signer_account_id();
        require!(
            self.id_of_account.contains_key(&signer),
            "No such issuer account exists"
        );
        let signer_account_id = self.id_of_account[&signer];
        let current_issuer_id = self.achievements[&_achievement_id].issuer;
        require!(
            current_issuer_id == signer_account_id,
            "Only issuer can update achievement owner"
        );
        self.achievements.get_mut(&_achievement_id).unwrap().owner =
            self.id_of_account[&_new_owner];
    }

    pub fn accept_achievement(&mut self, _achievement_id: u128) {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        require!(
            self.achievements[&_achievement_id].owner
                == self.id_of_account[&env::signer_account_id()],
            format!(
                "Only owner can accept this achievement, {}",
                _achievement_id
            )
        );
        self.achievements
            .get_mut(&_achievement_id)
            .unwrap()
            .is_accepted = true;
    }

    pub fn verify_achievement(&mut self, _achievement_id: u128) {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        let current_verifier = self.achievements[&_achievement_id].verifier;
        require!(
            current_verifier == self.id_of_account[&env::signer_account_id()],
            "Only verifier can verify an achievement"
        );
        require!(
            !self.achievements[&_achievement_id].is_verified,
            "Achievement is already verified"
        );
        self.achievements
            .get_mut(&_achievement_id)
            .unwrap()
            .is_verified = true;
        let achievement_balance = self.achievements[&_achievement_id].balance;
        let current_owner = self.account_of_id[&self.achievements[&_achievement_id].owner].clone();
        log!("Send {} NEAR to achievement owner", achievement_balance);
        Promise::new(current_owner).transfer(achievement_balance * NEAR);
        self.achievements
            .get_mut(&_achievement_id)
            .unwrap()
            .balance = 0;
    }

    pub fn split_achievement(
        &mut self,
        _achievement_id: u128,
        _new_owners: Vec<u128>,
        _new_achievement_ids: Vec<u128>,
    ) {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        require!(
            self.id_of_account
                .contains_key(&env::signer_account_id().clone()),
            "No such verifier exists"
        );
        let id_of_signer = self.id_of_account[&env::signer_account_id().clone()];
        let id_of_verifier = self.achievements[&_achievement_id].verifier;
        require!(
            id_of_signer == id_of_verifier,
            "Only verifier can split an achievement"
        );
        for new_id in _new_achievement_ids.iter() {
            require!(
                !self.achievements.contains_key(&new_id.clone()),
                format!("This achievement id already exists, {}", new_id)
            );
        }
        for new_owner in _new_owners.iter() {
            require!(
                self.account_of_id.contains_key(&new_owner.clone()),
                format!("No such owner exists, {}", new_owner)
            );
        }
        let number_of_new_owners = _new_owners.len() as u128;
        let balance_for_new_owners: Balance =
            self.achievements[&_achievement_id].balance * (number_of_new_owners);
        let return_money = self.achievements[&_achievement_id].balance
            - balance_for_new_owners * number_of_new_owners;
        log!("Returns {} NEAR to verifier", return_money);
        Promise::new(self.account_of_id[&id_of_verifier].clone()).transfer(return_money);
        for i in 0..number_of_new_owners {
            let mut new_achievement_data_address: Vector<u8> = Vector::new(b"m");
            for ch in self.achievements[&_achievement_id].data_address.iter() {
                new_achievement_data_address.push(&ch);
            }
            let current_issuer = self.achievements[&_achievement_id].issuer.clone();
            let current_owner = _new_owners.index(i as usize).clone();
            let current_achievement_id = _new_achievement_ids.index(i as usize).clone();
            let current_achievement_data = Achievement {
                achievement_id: _achievement_id.clone(),
                achievement_type: self.achievements[&_achievement_id].achievement_type,
                issuer: current_issuer.clone(),
                owner: current_owner.clone(),
                is_accepted: false,
                verifier: self.achievements[&_achievement_id].verifier,
                is_verified: false,
                data_address: new_achievement_data_address,
                balance: balance_for_new_owners,
            };
            self.achievements.insert(
                _new_achievement_ids.index(i as usize).clone(),
                current_achievement_data,
            );
            if !self
                .owners_achievements
                .contains_key(&_new_owners.index(i as usize).clone())
            {
                self.owners_achievements
                    .insert(_new_owners.index(i as usize).clone(), Vec::new());
            }
            if !self
                .issuers_achievements
                .contains_key(&current_issuer.clone())
            {
                self.issuers_achievements
                    .insert(current_issuer.clone(), Vec::new());
            }
            self.issuers_achievements
                .get_mut(&current_issuer)
                .unwrap()
                .push(current_achievement_id);
            self.owners_achievements
                .get_mut(&current_owner)
                .unwrap()
                .push(current_achievement_id);
        }
    }

    pub fn get_achievement_data(&self, _achievement_id: u128) -> IOAchievement {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        let mut data_address_as_str: String = String::new();
        for ch in self.achievements[&_achievement_id].data_address.iter() {
            data_address_as_str.push(ch as char);
        }

        let data = IOAchievement {
            achievement_id: self.achievements[&_achievement_id].achievement_id,
            achievement_type: self.achievements[&_achievement_id].achievement_type,
            issuer: self.achievements[&_achievement_id].issuer,
            owner: self.achievements[&_achievement_id].owner,
            is_accepted: self.achievements[&_achievement_id].is_accepted,
            verifier: self.achievements[&_achievement_id].verifier,
            is_verified: self.achievements[&_achievement_id].is_verified,
            data_address: data_address_as_str,
            balance: self.achievements[&_achievement_id].balance,
        };
        data
    }

    pub fn get_achievements_of_issuer(&self, _issuer: AccountId) -> Vec<u128> {
        require!(
            env::signer_account_id() == env::current_account_id()
                || env::signer_account_id() == _issuer,
            "Only operator or user can get this info"
        );
        require!(
            self.id_of_account.contains_key(&_issuer.clone()),
            "No such user exists"
        );
        let id_of_issuer = self.id_of_account[&_issuer.clone()];
        require!(
            self.issuers_achievements.contains_key(&id_of_issuer),
            "This user doesn't have any achievements"
        );
        let issuers_achievements = self.issuers_achievements[&id_of_issuer].clone();
        issuers_achievements
    }

    pub fn get_achievements_of_owner(&self, _owner: AccountId) -> Vec<u128> {
        require!(
            env::signer_account_id() == env::current_account_id()
                || env::signer_account_id() == _owner,
            "Only operator or issuer can get this info"
        );
        require!(
            self.id_of_account.contains_key(&_owner.clone()),
            "No such issuer exists"
        );
        let id_of_owner = self.id_of_account[&_owner.clone()];
        require!(
            self.owners_achievements.contains_key(&id_of_owner),
            "This issuer doesn't have any achievements"
        );
        let achievements_of_owner = self.owners_achievements[&id_of_owner].clone();
        achievements_of_owner
    }

    #[payable]
    pub fn replenish_achievement_balance(&mut self, _achievement_id: u128) {
        require!(
            self.achievements.contains_key(&_achievement_id),
            format!("No such achievement exists, {}", _achievement_id)
        );
        self.achievements.get_mut(&_achievement_id).unwrap().balance += env::attached_deposit();
    }
}
