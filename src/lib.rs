use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base58CryptoHash, Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};
use std::collections::{HashMap, HashSet};
mod bounties;
mod policy;
mod proposols;
mod types;
mod utils;
use crate::bounties::*;
use crate::policy::*;
use crate::proposols::*;
use crate::types::*;
use crate::utils::*;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    config: LazyOption<Config>,
    policy: LazyOption<VersionedPolicy>,
    /// locked $NEAR
    locked_amount: Balance,
    token: FungibleToken,
    token_metadata: LazyOption<FungibleTokenMetadata>,
    proposals: LookupMap<u64, VersionedProposal>,
    last_proposal_id: u64,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    Config,
    Policy,
    Token,
    TokenMetadata,
    Proposals,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(config: Config, policy: VersionedPolicy, metadata: FungibleTokenMetadata) -> Self {
        Self {
            config: LazyOption::new(StorageKeys::Config, Some(&config)),
            policy: LazyOption::new(StorageKeys::Policy, Some(&policy)),
            locked_amount: 0,
            token: FungibleToken::new(StorageKeys::Token),
            token_metadata: LazyOption::new(StorageKeys::TokenMetadata, Some(&metadata)),
            last_proposal_id: 0,
            proposals: LookupMap::new(StorageKeys::Proposals),
        }
    }
}

impl Contract {
    pub fn get_user_weight(&self, account_id: &AccountId) -> Balance {
        self.token.accounts.get(account_id).unwrap_or(0)
    }
    pub fn internal_user_info(&self) -> UserInfo {
        let account_id = env::predecessor_account_id();
        UserInfo {
            account_id: account_id.clone(),
            amount: self.get_user_weight(&account_id),
        }
    }
}
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_basics() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());

        println!("test");
    }
}
