use crate::*;

pub type ProposalId = i128;
pub type ContributionId = U128;
pub type Date = u64;


//Metadata need for init the contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ForMyFutureContractMetadata { 
    pub name: String, //Only for initialization purposes
}


//This structure has the general information about proposal
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalMetadata {        
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

//Main contract structure, link the proposal metadata with an owner
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: ProposalId,
    pub owner: AccountId,
    pub metadata: ProposalMetadata,
    pub image: String, //the main image, it will be get from the first array position
    pub status: i8 //the status will be represent by an integer from 0 to 2
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    pub id: ContributionId,
    pub from: AccountId,
    pub by: AccountId,
    pub proposal_id: ProposalId,
    pub amount: Balance,
    pub image: String //Main proposal link image
}

//Trait for view method
pub trait ContractMetadata {
    fn contract_metadata(&self) -> ForMyFutureContractMetadata;
}

//Shows the contract metadata in a view method
#[near_bindgen]
impl ContractMetadata for Contract {
    fn contract_metadata(&self) -> ForMyFutureContractMetadata {
        return self.metadata.get().unwrap();
    }
}