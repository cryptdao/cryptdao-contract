use crate::*;

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
    pub fn metadata(&self) -> DaoMeta {
        DaoMeta {
            name: self.config.get().unwrap().name.clone(),
            headcount: self.headcount,
        }
    }

    pub fn get_citizen(&self, account_id: AccountId) -> Option<Citizen> {
        log!("found {}", account_id);
        match self.citizens.get(&account_id) {
            Some(VersionedCitizen::Current(citizen)) => Some(citizen),
            _ => None,
        }
    }

    pub fn get_policy(&self) -> Policy {
        self.policy.get().unwrap().to_policy().clone()
    }

    pub fn get_proposal(&self, id: u64) -> ProposalOutput {
        let proposal = self.proposals.get(&id).expect("ERR_NO_PROPOSAL");
        ProposalOutput {
            id,
            proposal: proposal.into(),
        }
    }
}
