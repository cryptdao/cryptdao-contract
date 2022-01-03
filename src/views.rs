use crate::*;
use std::cmp::min;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct DaoMeta {
    pub name: String,
    pub headcount: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    pub id: u64,
    #[serde(flatten)]
    pub proposal: Proposal,
}

#[near_bindgen]
impl Contract {
    /// get the current metadata of the dao.
    pub fn metadata(&self) -> DaoMeta {
        DaoMeta {
            name: self.config.get().unwrap().name.clone(),
            headcount: self.headcount,
        }
    }

    /// get citizen with given account id.
    pub fn get_citizen(&self, account_id: AccountId) -> Option<Citizen> {
        log!("found {}", account_id);
        match self.citizens.get(&account_id) {
            Some(VersionedCitizen::Current(citizen)) => Some(citizen),
            _ => None,
        }
    }

    /// get current policy.
    pub fn get_policy(&self) -> Policy {
        self.policy.get().unwrap().to_policy().clone()
    }

    /// get the proposal with given id.
    pub fn get_proposal(&self, id: u64) -> ProposalOutput {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        ProposalOutput {
            id,
            proposal: proposal.into(),
        }
    }

    /// get all proposals
    pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput> {
        (from_index..min(self.last_proposal_id, from_index + limit))
            .filter_map(|id| {
                self.proposals.get(&id).map(|proposal| ProposalOutput {
                    id,
                    proposal: proposal.into(),
                })
            })
            .collect()
    }
}
