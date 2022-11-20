use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, ext_contract};
use std::collections::HashMap;
// use std::convert::{TryFrom, TryInto};

type MyHash = [u8; 32];
type HashedData = [MyHash; 2];

// // FIXME: Should be not a function, but a const global variable or SBT struct member
// fn is_achievements_contract_account(acc: AccountId) -> bool {
//     let achievements_contract_account = AccountId::try_from("sbt.souldev.testnet".to_string()).unwrap();
//     achievements_contract_account == acc
// }

#[ext_contract(ext_self)]
trait ExtSelf {    
    pub fn get_user_id(&self, account: &AccountId) -> u128;
}


#[derive(Default, BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct Soul {
    soul_id: u128,
    git_hash: MyHash,
    email_hash: MyHash,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct SBT {
    // achievements_contract_account: AccountId,

    // TODO: Find a way to store achievements contract account: problem: There is no default value for AccountId
    souls_: HashMap<u128, Soul>,
    soul_id_of_account_: HashMap<AccountId, u128>,
    account_of_soul_id: HashMap<u128, AccountId>,
    minted_not_claimed: HashMap<u128, bool>,
}

#[near_bindgen]
impl SBT {
    // Public read-only method: Returns the counter value.
    pub fn mint(&mut self, new_id: u128, account: &AccountId) {
        require!(self.souls_[&new_id].soul_id != 0, "Soul exists");
        require!(!self.minted_not_claimed[&new_id], "Soul is already minted");
        require!(
            env::signer_account_id() == env::current_account_id(),
            "Only this contract can mint SBT"
        );
        *self.soul_id_of_account_.get_mut(account).unwrap() = new_id;
        *self.minted_not_claimed.get_mut(&new_id).unwrap() = true;
    }

    pub fn claim(&mut self, new_git_hash: &MyHash, new_email_hash: &MyHash) {
        let msg_sender = env::signer_account_id();
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        require!(
            self.minted_not_claimed[&msg_sender_soul_id],
            "Soul must be minted to be able to claim it"
        );
        self.souls_.get_mut(&msg_sender_soul_id).unwrap().email_hash = *new_email_hash;
        self.souls_.get_mut(&msg_sender_soul_id).unwrap().git_hash = *new_git_hash; // TODO: Is it correct? Does it store value, not pointer?
        *self
            .account_of_soul_id
            .get_mut(&msg_sender_soul_id)
            .unwrap() = msg_sender;
        self.minted_not_claimed.remove(&msg_sender_soul_id);
    }

    pub fn get_user_id(&self, account: &AccountId) -> u128 {
        self.soul_id_of_account_[account]
    }

    pub fn burn(&mut self) {
        let msg_sender = env::signer_account_id();
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        require!(
            self.account_of_soul_id[&msg_sender_soul_id] == msg_sender,
            "Soul must exist to be able to burn it"
        );

        self.soul_id_of_account_.remove(&msg_sender);
        self.account_of_soul_id.remove(&msg_sender_soul_id);
        self.souls_.remove(&msg_sender_soul_id);
    }

    pub fn has_soul(&self, account: &AccountId) -> bool {
        let account_soul_id = self.soul_id_of_account_[account];
        self.account_of_soul_id[&account_soul_id] != *account
    }

    // Soul doesn't have Serialization. TODO: Decide - implement Seralization or leave with next function?
    // pub fn get_soul(&self, account: &AccountId) -> Soul {
    //     let account_soul_id = self.soul_id_of_account_[&account];
    //     let msg_sender = env::signer_account_id();
    //     let operator = env::current_account_id();
    //     require!(
    //         operator == *account || msg_sender == *account,
    //         "Only this contract or user can access this data"
    //     );
    //     require!(
    //         self.account_of_soul_id[&account_soul_id] == *account,
    //         "Soul must exist to get it"
    //     );
    //     return self.souls_[&account_soul_id];
    // }

    pub fn get_hashed_data(&self) -> HashedData {
        let msg_sender = env::signer_account_id();
        let msg_sender_soul_id = self.soul_id_of_account_[&msg_sender];
        require!(
            self.account_of_soul_id[&msg_sender_soul_id] == msg_sender,
            "Soul must exist to get it's data"
        );
        let mut users_data = [[0; 32]; 2];

        users_data[0] = self.souls_[&msg_sender_soul_id].git_hash;
        users_data[1] = self.souls_[&msg_sender_soul_id].email_hash;
        users_data
    }
}
