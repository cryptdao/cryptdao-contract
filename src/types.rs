use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// Name of the DAO.
    pub name: String,
    /// Purpose of this DAO.
    pub purpose: String,
    /// Generic metadata. Can be used by specific UI to store additional data.
    /// This is not used by anything in the contract.
    pub metadata: Base64VecU8,
}

#[cfg(test)]
impl Config {
    pub fn test_config() -> Self {
        Self {
            name: "Test".to_string(),
            purpose: "to test".to_string(),
            metadata: Base64VecU8(vec![]),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    /// Create a new proposal.
    AddProposal,
    /// Remove a proposal.
    RemoveProposal,
    /// Vote on a proposal.
    VoteApprove,
    /// Reject a proposal.
    VoteReject,
    /// Remove a vote.
    VoteRemove,
    /// Finalize a proposal.
    Finalize,
    /// Move to the next stage.
    MoveToHub,
}

impl Action {
    pub fn label(&self) -> String {
        format!("{:?}", self)
    }
}
