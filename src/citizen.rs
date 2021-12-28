/// Versioned policy.
use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Citizen {
    /// Name of the citizen.
    account_id: AccountId,
    /// show citizen role.
    role_name: String,
    /// show citizen join date
    joined: U64,
}

impl Citizen {
    pub fn new(account_id: AccountId, role_name: String) -> Self {
        Self {
            account_id,
            role_name,
            joined: get_timestamp(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum VersionedCitizen {
    /// Default policy with given accounts as council.
    Current(Citizen),
}
