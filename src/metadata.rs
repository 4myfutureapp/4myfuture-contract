use crate::*;

pub type ProposalId = U128;
pub type ContributionId = U128;
pub type Date = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ForMyFutureContractMetadata {
    owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalMetadata {
    id: ProposalId,
    title: String,
    description: String,
    goal: U128,
    funds: u128,
    init_date: Date,
    finish_date: Date,
    institution_link: String,
    pensum_link: String,
    images: Vec<String>

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    id: ProposalId,
    owner: AccountId,
    metadata: ProposalMetadata,
    image: String,
    status: i8
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    id: ContributionId,
    from: AccountId,
    by: AccountId,
    proposal_id: ProposalId,
    amount: Balance,
    image: String
}

pub trait ContractMetadata {
    fn contract_metadata(&self) -> ForMyFutureContractMetadata;
}

#[near_bindgen]
impl ContractMetadata for Contract {
    fn contract_metadata(&self) -> ForMyFutureContractMetadata {
        return self.metadata.get().unwrap();
    }
}