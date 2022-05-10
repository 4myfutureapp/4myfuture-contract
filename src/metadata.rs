use crate::*;

pub type ProposalId = U128;
pub type ContributionId = U128;
pub type Date = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ForMyFutureContractMetadata {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalMetadata {
    pub id: ProposalId,
    pub title: String,
    pub description: String,
    pub goal: U128,
    pub funds: u128,
    pub init_date: Date,
    pub finish_date: Date,
    pub institution_link: String,
    pub pensum_link: String,
    pub images: Vec<String>

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: ProposalId,
    pub owner: AccountId,
    pub metadata: ProposalMetadata,
    pub image: String,
    pub status: i8
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    pub id: ContributionId,
    pub from: AccountId,
    pub by: AccountId,
    pub proposal_id: ProposalId,
    pub amount: Balance,
    pub image: String
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