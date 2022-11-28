// SPDX-License-Identifier: MIT
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, require, AccountId};
use std::collections::HashMap;
use std::str;

type MyHash = [u8; 32];
type HashedData = [String; 2];

#[derive(Default, BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct Soul {
    soul_id: u128,
    git_hash: MyHash,
    email_hash: MyHash,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct SBT {
    souls_: HashMap<u128, Soul>,
    soul_id_of_account_: HashMap<AccountId, u128>,
    account_of_soul_id: HashMap<u128, AccountId>,
    minted_not_claimed: HashMap<u128, bool>,
}

#[near_bindgen]
impl SBT {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            souls_: HashMap::new(),
            soul_id_of_account_: HashMap::new(),
            account_of_soul_id: HashMap::new(),
            minted_not_claimed: HashMap::new(),
        }
    }

    pub fn mint(&mut self, new_id: u128, account: AccountId) {
        require!(!self.has_soul(&account), "Soul exists");
        require!(
            !self.soul_is_minted_not_claimed(&account),
            "Soul is already minted"
        );
        require!(
            env::signer_account_id() == env::current_account_id(),
            "Only this contract can mint SBT"
        );
        self.soul_id_of_account_.insert(account, new_id);
        self.minted_not_claimed.insert(new_id, true);
    }

    pub fn claim(&mut self, new_git_hash: &String, new_email_hash: &String) {
        let msg_sender = env::signer_account_id();
        require!(
            self.soul_is_minted_not_claimed(&msg_sender),
            "Soul must be minted to be able to claim it"
        );
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        log!(
            "New git hash is {}, new email hash is {}",
            new_git_hash,
            new_email_hash
        );
        require!(
            new_git_hash.chars().count() == 32,
            "Given hash must be 32 character length"
        );
        require!(
            new_email_hash.chars().count() == 32,
            "Given hash must be 32 character length"
        );
        let mut hashed_git: MyHash = [0; 32];
        for i in 0..32 {
            hashed_git[i] = new_git_hash.as_bytes()[i];
        }
        let mut hashed_email: MyHash = [0; 32];
        for i in 0..32 {
            hashed_email[i] = new_email_hash.as_bytes()[i];
        }
        
        let current_soul = Soul {
            soul_id: msg_sender_soul_id,
            git_hash: hashed_git,
            email_hash: hashed_email,
        };
        self.souls_.remove(&msg_sender_soul_id);
        self.account_of_soul_id.remove(&msg_sender_soul_id);
        self.souls_.insert(msg_sender_soul_id, current_soul);
        self.account_of_soul_id
            .insert(msg_sender_soul_id, msg_sender);
    }

    pub fn get_user_id(&self, account: &AccountId) -> u128 {
        require!(self.has_soul(account), "No user found");
        self.soul_id_of_account_[account]
    }

    pub fn get_account_id(&self, user_id: u128) -> AccountId {
        require!(
            self.account_of_soul_id.contains_key(&user_id)
                && self.has_soul(&self.account_of_soul_id[&user_id]),
            "No user found"
        );
        self.account_of_soul_id[&user_id].clone()
    }

    pub fn burn(&mut self) {
        let msg_sender = env::signer_account_id();
        require!(
            self.has_soul(&msg_sender),
            "Soul must exist to be able to burn it"
        );
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        self.soul_id_of_account_.remove(&msg_sender);
        self.account_of_soul_id.remove(&msg_sender_soul_id);
        self.souls_.remove(&msg_sender_soul_id);
    }

    pub fn has_soul(&self, account: &AccountId) -> bool {
        if !self.soul_id_of_account_.contains_key(account) {
            return false;
        }
        if !self.souls_.contains_key(&self.soul_id_of_account_[account]) {
            return false;
        }
        true
    }

    pub fn soul_is_minted_not_claimed(&self, account: &AccountId) -> bool {
        if !self.soul_id_of_account_.contains_key(&account) {
            return false;
        }
        let msg_sender_soul_id = self.soul_id_of_account_[&account];
        if !self.minted_not_claimed.contains_key(&msg_sender_soul_id) {
            return false;
        }
        self.minted_not_claimed[&msg_sender_soul_id]
    }

    pub fn ping(&self) -> bool {
        true
    }

    pub fn ping_string(&self) -> String {
        "I'm okey".to_string()
    }

    pub fn get_hashed_data(&self) -> HashedData {
        let msg_sender = env::signer_account_id();
        require!(
            self.has_soul(&msg_sender),
            "Soul must exist to get it's data"
        );
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        [
            String::from_utf8(self.souls_[&msg_sender_soul_id].git_hash.to_vec()).unwrap(),
            String::from_utf8(self.souls_[&msg_sender_soul_id].email_hash.to_vec()).unwrap(),
        ]
    }
}
