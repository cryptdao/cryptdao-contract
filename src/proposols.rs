/// Status of a proposal.
use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    /// New proposal.
    New,
    /// Proposal is waiting for council to vote.
    InProgress,
    /// If quorum voted yes, this proposal is successfully approved.
    Approved,
    /// If quorum voted no, this proposal is rejected. Bond is returned.
    Rejected,
    /// If quorum voted to remove (e.g. spam), this proposal is rejected and bond is not returned.
    /// Interfaces shouldn't show removed proposals.
    Removed,
    /// Expired after period of time.
    Expired,
    /// If proposal was moved to Hub or somewhere else.
    Moved,
    /// If proposal has failed when finalizing. Allowed to re-finalize again to either expire or approved.
    Failed,
}

/// Function call arguments.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct ActionCall {
    method_name: String,
    args: Base64VecU8,
    deposit: U128,
    gas: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wash32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct VoteOption {
    /// Option id of the Vote
    pub name: String,
    /// Option
    pub content: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct VoteKind {
    pub options: Vec<VoteOption>,
    pub votes: HashMap<AccountId, Vec<VoteOption>>,
    pub option_counts: HashMap<String, Balance>,
}

/// Kinds of proposals, doing different action.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ProposalKind {
    /// Change the DAO config.
    ChangeConfig { config: Config },
    /// Change the full policy.
    ChangePolicy { policy: VersionedPolicy },
    /// Add member to given role in the policy. This is short cut to updating the whole policy.
    AddMemberToRole { member_id: AccountId, role: String },
    /// Remove member to given role in the policy. This is short cut to updating the whole policy.
    RemoveMemberFromRole { member_id: AccountId, role: String },
    /// Calls `receiver_id` with list of method names in a single promise.
    /// Allows this contract to execute any arbitrary set of actions in other contracts.
    FunctionCall {
        receiver_id: AccountId,
        actions: Vec<ActionCall>,
    },
    /// Upgrade this contract with given hash from blob store.
    UpgradeSelf { hash: Base58CryptoHash },
    /// Upgrade another contract, by calling method with the code from given hash from blob store.
    UpgradeRemote {
        receiver_id: AccountId,
        method_name: String,
        hash: Base58CryptoHash,
    },
    /// Transfers given amount of `token_id` from this DAO to `receiver_id`.
    /// If `msg` is not None, calls `ft_transfer_call` with given `msg`. Fails if this base token.
    /// For `ft_transfer` and `ft_transfer_call` `memo` is the `description` of the proposal.
    Transfer {
        /// Can be "" for $NEAR or a valid account id.
        #[serde(with = "serde_with::rust::string_empty_as_none")]
        token_id: Option<AccountId>,
        receiver_id: AccountId,
        amount: U128,
        msg: Option<String>,
    },
    /// Just a signaling vote, with no execution.
    Vote(VoteKind),
}

impl ProposalKind {
    pub fn label(&self) -> &str {
        match self {
            ProposalKind::ChangeConfig { .. } => "ChangeConfig",
            ProposalKind::ChangePolicy { .. } => "ChangePolicy",
            ProposalKind::AddMemberToRole { .. } => "AddMemberToRole",
            ProposalKind::RemoveMemberFromRole { .. } => "RemoveMemberFromRole",
            ProposalKind::FunctionCall { .. } => "FunctionCall",
            ProposalKind::UpgradeSelf { .. } => "UpgradeSelf",
            ProposalKind::UpgradeRemote { .. } => "UpgradeRemote",
            ProposalKind::Transfer { .. } => "Transfer",
            ProposalKind::Vote { .. } => "Vote",
        }
    }
}

/// Votes recorded in the proposal.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Approve = 0x0,
    Reject = 0x1,
    Remove = 0x2,
}

/// Proposal that are sent to this DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    /// Original proposer.
    pub proposer: AccountId,
    /// Proposal title.
    pub title: String,
    /// Description of this proposal.
    pub description: String,
    /// Kind of proposal with relevant information.
    pub kind: ProposalKind,
    /// Current status of the proposal.
    pub status: ProposalStatus,
    pub submission_time: u64,
    /// Voting period start time.
    pub proposal_start_time: u64,
    /// Voting period end time.
    pub proposal_end_time: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum VersionedProposal {
    Default(Proposal),
}

impl From<VersionedProposal> for Proposal {
    fn from(v: VersionedProposal) -> Self {
        match v {
            VersionedProposal::Default(p) => p,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalInput {
    /// Proposal title.
    pub title: String,
    /// Description of this proposal.
    pub description: String,
    /// Kind of proposal with relevant information.
    pub kind: ProposalKind,
    /// start time of the voting period.
    pub proposal_start_time: u64,
    /// end time of the voting period.
    pub proposal_end_time: u64,
}

impl From<ProposalInput> for Proposal {
    fn from(input: ProposalInput) -> Self {
        Self {
            title: input.title,
            proposer: env::predecessor_account_id(),
            description: input.description,
            kind: input.kind,
            status: ProposalStatus::InProgress,
            submission_time: get_timestamp(),
            proposal_start_time: input.proposal_start_time,
            proposal_end_time: input.proposal_end_time,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_proposal(&mut self, proposal: ProposalInput) -> u64 {
        let policy = self.policy.get().unwrap().to_policy();
        assert!(
            env::attached_deposit() >= policy.proposal_bond.0,
            "ERR_MIN_BOND"
        );

        match &proposal.kind {
            ProposalKind::ChangePolicy { policy } => match policy {
                VersionedPolicy::Current(_) => {}
                _ => panic!("ERR_INVALID_POLICY"),
            },
            ProposalKind::Transfer { token_id, msg, .. } => {
                assert!(
                    !(token_id.is_none()) || msg.is_none(),
                    "ERR_INVALID_TRANSFER"
                );
            }
            ProposalKind::AddMemberToRole { member_id, role } => {
                assert!(
                    !(member_id.as_str().is_empty() || role.is_empty()),
                    "ERR_INVALID_ADD_MEMBER"
                );
            }
            _ => panic!("ERR_UNSUPPORTED_PROPOSAL"),
        };
        assert!(
            policy
                .can_execute_action(self.internal_user_info(), &Action::AddProposal)
                .1,
            "ERR_PERMISSION_DENIED"
        );
        let id = self.last_proposal_id;
        self.proposals
            .insert(&id, &VersionedProposal::Default(proposal.into()));
        self.last_proposal_id += 1;
        self.locked_amount += env::attached_deposit();
        id
    }
}
