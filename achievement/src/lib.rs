use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, ext_contract, require, AccountId, Promise};
use std::collections::HashMap;
use std::vec::Vec;
#[ext_contract(sbt_contract)]
trait SBTContractInterface { 
    fn get_user_id(_user: AccountId) -> u128;
}

// fn is_sbt_contract_account(acc: AccountId) -> bool {
//     let achievements_contract_account = AccountId::try_from("sbt.souldev.testnet".to_string()).unwrap();
//     achievements_contract_account == acc
// }

fn get_user_id(_user: AccountId) -> u128 {
    // TODO: Here we should call sbt contract and get id of our user from there
    42
}

fn get_user_account(_user_uid: u128) -> AccountId {
    // TODO: Here we should call sbt contract and get id of our user from there
    AccountId::try_from("sbt.souldev.testnet".to_string()).unwrap()
}

#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Achievement {
    achievement_id: u128,
    achievement_type: u128,
    issuer: u128,
    owner: u128,
    is_accepted: bool,
    verifier: u128,
    is_verified: bool,
    data_address: String,
    balance: u128,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AchievementToken {
    // TODO: Find a way to store achievements contract account: problem: There is no default value for AccountId
    achievements: HashMap<u128, Achievement>,
    issuers_achievements: HashMap<u128, Vec<u128>>,
    users_achievements: HashMap<u128, Vec<u128>>,
}

#[near_bindgen]
impl AchievementToken {
    #[payable]
    fn mint(&mut self, _achievement_data: Achievement) {
        require!(
            self.achievements[&_achievement_data.achievement_id].issuer != 0,
            "Achievement id already exists"
        );
        require!(
            env::signer_account_id() == env::current_account_id()
                || get_user_id(env::signer_account_id()) == _achievement_data.issuer,
            "Only you can be an issuer"
        );
        require!(
            env::attached_deposit() >= _achievement_data.balance,
            "Not enough balance attached"
        );

        let return_money = env::attached_deposit() - _achievement_data.balance;
        Promise::new(env::signer_account_id()).transfer(return_money);

        let current_id = _achievement_data.achievement_id;
        *self.achievements.get_mut(&current_id).unwrap() = _achievement_data;
        self.issuers_achievements
            .get_mut(&self.achievements[&current_id].issuer)
            .unwrap()
            .push(current_id);
        self.users_achievements
            .get_mut(&self.achievements[&current_id].owner)
            .unwrap()
            .push(current_id);
    }

    fn burn(&mut self, _achievement_id: u128) {
        require!(
            self.achievements[&_achievement_id].issuer != 0,
            "Achievement id already exists"
        );
        require!(
            get_user_id(env::signer_account_id()) == self.achievements[&_achievement_id].issuer,
            "Only you can be an issuer"
        );
        let current_issuer_id = self.achievements[&_achievement_id].issuer;
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
        for i in 0..self.users_achievements[&current_owner_id].len() {
            if _achievement_id == self.users_achievements[&current_owner_id][i] {
                self.users_achievements
                    .get_mut(&current_owner_id)
                    .unwrap()
                    .remove(i);
                break;
            }
        }  
        self.achievements.remove(&_achievement_id);      
    }

    fn update_owner(&mut self, _achievement_id: u128, _new_owner: AccountId) {
        let new_account_id = get_user_id(_new_owner);
        require!(
            get_user_id(env::signer_account_id()) == self.achievements[&_achievement_id].issuer,
            "Only issuer can change an owner"
        );
        require!(
            self.achievements[&_achievement_id].owner == 0,
            "Owner of this achievement can not be changed"
        );
        self.achievements.get_mut(&_achievement_id).unwrap().owner = new_account_id;
    }

    fn accept_achievement(&mut self, _achievement_id: u128) {
        let current_account_id = get_user_id(env::signer_account_id());
        require!(
            current_account_id == self.achievements[&_achievement_id].owner,
            "Only owner can accept this achievement"
        );
        self.achievements.get_mut(&_achievement_id).unwrap().is_accepted = true;
    }

    fn verify_achievement(&mut self, _achievement_id: u128) {
        let current_account_id = get_user_id(env::signer_account_id());
        let current_verifier = self.achievements[&_achievement_id].verifier;
        require!(
            current_account_id == current_verifier,
            "Only verifier can verify an achievement"
        );
        require!(
            !self.achievements[&_achievement_id].is_verified,
            "Achievement already verified"
        );

        let verifier_account = get_user_account(current_verifier);
        Promise::new(verifier_account).transfer(self.achievements[&_achievement_id].balance);
        self.achievements.get_mut(&_achievement_id).unwrap().is_verified = true;
    }

    // fn split_achievement(&mut self, _achievement_id: u128) {
// TODO: Do we actually need it?
    // }

    fn get_achievement_data(&self, _achievement_id: u128) -> Achievement {
        self.achievements[&_achievement_id].clone()
    }

    #[payable]
    fn replenish_achievement_balance(&mut self, _achievement_id: u128) {
        self.achievements.get_mut(&_achievement_id).unwrap().balance += env::attached_deposit();
    }

}

