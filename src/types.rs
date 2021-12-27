use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// Name of the DAO.
    pub name: String,
    /// Purpose of this DAO.
    pub purpose: String,
}

impl Config {
    pub fn test_config() -> Self {
        Self {
            name: "Test".to_string(),
            purpose: "to test".to_string(),
        }
    }

    pub fn new(name: String, purpose: String) -> Self {
        Self {
            name: name,
            purpose: purpose,
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
