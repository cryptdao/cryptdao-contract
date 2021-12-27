use crate::*;

/// How the voting policy votes get weigthed.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    /// Using token amounts and total delegated at the moment.
    TokenWeight,
    /// Weight of the group role. Roles that don't have scoped group are not supported.
    RoleWeight,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum WeightOrRatio {
    Weight(U128),
    Ratio(u64, u64),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub enum RoleKind {
    /// Matches everyone, who is not matched by other roles.
    Everyone,
    /// Member greater or equal than given balance. Can use `1` as non-zero balance.
    Member(U128),
    /// Set of accounts.
    Group(HashSet<AccountId>),
}

impl RoleKind {
    pub fn match_user(&self, user: &UserInfo) -> bool {
        match self {
            RoleKind::Everyone => true,
            RoleKind::Member(amount) => user.amount >= amount.0,
            RoleKind::Group(accounts) => accounts.contains(&user.account_id),
        }
    }
}

/// Defines configuration of the vote.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy {
    /// Kind of weight to use for votes.
    pub weight_kind: WeightKind,
    /// Minimum number required for vote to finalize.
    /// If weight kind is TokenWeight - this is minimum number of tokens required.
    ///     This allows to avoid situation where the number of staked tokens from total supply is too small.
    /// If RoleWeight - this is minimum umber of votes.
    ///     This allows to avoid situation where the role is got too small but policy kept at 1/2, for example.
    pub quorum: U128,
    /// How many votes to pass this vote.
    pub threshold: WeightOrRatio,
}

impl Default for VotePolicy {
    fn default() -> Self {
        VotePolicy {
            weight_kind: WeightKind::RoleWeight,
            quorum: U128(0),
            threshold: WeightOrRatio::Ratio(1, 2),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Role {
    /// Name of the role to display to the user.
    pub name: String,
    /// Kind of the role: defines which users this permissions apply.
    pub kind: RoleKind,
    /// Set of actions on which proposals that this role is allowed to execute.
    /// <proposal_kind>:<action>
    pub permissions: HashSet<String>,
    /// For each proposal kind, defines voting policy.
    pub vote_policy: HashMap<String, VotePolicy>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    /// List of roles and permissions for them in the current policy.
    pub roles: Vec<Role>,
    /// Default vote policy. Used when given proposal kind doesn't have special policy.
    pub default_vote_policy: VotePolicy,
    /// Expiration period for proposals.
    pub proposal_period: U64,
    /// Proposal bond.
    pub proposal_bond: U128,
}

/// Versioned policy.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum VersionedPolicy {
    /// Default policy with given accounts as council.
    Current(Policy),
}

impl Policy {
    pub fn default_policy(council: Vec<AccountId>) -> Self {
        Self {
            roles: vec![
                Role {
                    name: "all".to_string(),
                    kind: RoleKind::Everyone,
                    permissions: vec![Action::AddProposal.label()].into_iter().collect(),
                    vote_policy: HashMap::default(),
                },
                Role {
                    name: "member".to_string(),
                    kind: RoleKind::Member(U128(1)),
                    permissions: vec![
                        Action::AddProposal.label(),
                        Action::VoteApprove.label(),
                        Action::VoteReject.label(),
                    ]
                    .into_iter()
                    .collect(),
                    vote_policy: HashMap::default(),
                },
                Role {
                    name: "council".to_string(),
                    kind: RoleKind::Group(council.into_iter().collect()),
                    permissions: vec![
                        Action::AddProposal.label(),
                        Action::VoteApprove.label(),
                        Action::VoteReject.label(),
                        Action::VoteRemove.label(),
                        Action::Finalize.label(),
                    ]
                    .into_iter()
                    .collect(),
                    vote_policy: HashMap::default(),
                },
            ],
            default_vote_policy: VotePolicy::default(),
            proposal_period: U64::from(1_000_000_000 * 60 * 60 * 24 * 7),
            proposal_bond: U128(10u128.pow(24)),
        }
    }
}
pub struct UserInfo {
    pub account_id: AccountId,
    pub amount: Balance,
}

impl VersionedPolicy {
    pub fn to_policy(self) -> Policy {
        match self {
            VersionedPolicy::Current(policy) => policy,
            _ => unimplemented!(),
        }
    }
}

impl Policy {
    fn get_user_roles(&self, user: UserInfo) -> HashMap<String, &Role> {
        let mut roles = HashMap::default();
        for role in self.roles.iter() {
            if role.kind.match_user(&user) {
                roles.insert(role.name.clone(), role);
            }
        }
        roles
    }

    pub fn can_execute_action(
        &self,
        user: UserInfo,
        proposal_kind: &ProposalKind,
        action: &Action,
    ) -> (Vec<&Role>, bool) {
        let roles = self.get_user_roles(user);
        let mut allowed = false;
        let allowed_roles = roles
            .into_iter()
            .filter_map(|(name, role)| {
                let allowed_role = role.permissions.contains(&format!(
                    "{}:{}",
                    proposal_kind.label(),
                    action.label()
                ));
                allowed = allowed || allowed_role;
                if allowed_role {
                    Some(role)
                } else {
                    None
                }
            })
            .collect();
        (allowed_roles, allowed)
    }
}
