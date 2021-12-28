use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base58CryptoHash, Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};
use std::collections::{HashMap, HashSet};
mod bounties;
mod citizen;
mod policy;
mod proposols;
mod treasury;
mod types;
mod utils;
mod views;
use crate::citizen::*;
use crate::policy::*;
use crate::proposols::*;
use crate::treasury::*;
use crate::types::*;
use crate::utils::*;
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    config: LazyOption<Config>,
    policy: LazyOption<VersionedPolicy>,
    token: FungibleToken,
    token_metadata: LazyOption<FungibleTokenMetadata>,
    proposals: LookupMap<u64, VersionedProposal>,
    last_proposal_id: u64,
    citizens: LookupMap<AccountId, VersionedCitizen>,
    headcount: u64,
    treasury: VersionedTreasury,
    locked_amount: Balance,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    Config,
    Policy,
    Token,
    TokenMetadata,
    Proposals,
    Treasury,
    Citizens,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = include_str!("../res/logo.svg");
const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

fn default_metadata() -> FungibleTokenMetadata {
    FungibleTokenMetadata {
        spec: "ft-1.0.0".to_string(),
        name: "CryptDAO".to_string(),
        symbol: "DAO".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 24,
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String, purpose: String, council: Vec<AccountId>) -> Self {
        let mut citizens = LookupMap::new(StorageKeys::Citizens);
        council.iter().for_each(|x| {
            citizens.insert(
                x,
                &VersionedCitizen::Current(Citizen::new(x.clone(), "council".to_string())),
            );
        });
        let mut headcount = council.len() as u64;
        let owner_id = env::signer_account_id();
        let mut this = Self {
            config: LazyOption::new(StorageKeys::Config, Some(&Config::new(name, purpose))),
            policy: LazyOption::new(
                StorageKeys::Policy,
                Some(&VersionedPolicy::Current(Policy::default_policy(
                    council.clone(),
                ))),
            ),
            token: FungibleToken::new(StorageKeys::Token),
            token_metadata: LazyOption::new(StorageKeys::TokenMetadata, Some(&default_metadata())),
            last_proposal_id: 0,
            proposals: LookupMap::new(StorageKeys::Proposals),
            treasury: VersionedTreasury::Current(Treasury::default()),
            citizens: citizens,
            headcount: headcount,
            locked_amount: 0,
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, TOTAL_SUPPLY);
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

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
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk_sim::to_yocto;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn create_proposal(context: &mut VMContextBuilder, contract: &mut Contract) -> u64 {
        //testing_env!(context.attached_deposit(to_yo))
        testing_env!(context.attached_deposit(to_yocto(1)).build());
        contract.add_proposal(ProposalInput {
            description: "test".to_string(),
            kind: ProposalKind::Transfer {
                token_id: None,
                receiver_id: accounts(2).into(),
                amount: U128(to_yocto("100")),
                msg: None,
            },
        })
    }

    #[test]
    fn test_basics() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(1)).build());
        let mut contract = Contract::new(
            config::test_config(),
            VersionPolicy::Default(vec![accounts(1).into()]),
            FungibleTokenMetadata::new(
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            ),
        );
        let id = create_proposal(&mut context, &mut contract);
        println!("test");
    }
}
