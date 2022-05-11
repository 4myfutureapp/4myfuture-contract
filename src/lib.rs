use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey,
    CryptoHash, Gas, PanicOnDefault, Promise,
};
use std::collections::HashMap;

mod metadata;
mod proposal;
mod internal;
mod enumeration;


use crate::metadata::*;
use crate::proposal::*;
use crate::internal::*;
use crate::enumeration::*;

const ONE_NEAR: Balance = 1000000000000000000000000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId, //Contract owner
    pub proposal_per_owner: LookupMap<AccountId, Proposal>, //Link owners with Proposal
    pub proposal_by_id: UnorderedMap<ProposalId, Proposal>, //Link proposals ID with Proposal
    pub proposal_metadata_by_id: LookupMap<ProposalId, ProposalMetadata>, //Link Proposals ID with Proposal Metadata  
    pub contributions_per_user: LookupMap<AccountId, Contribution>, //Link users and contributions
    pub contributions_per_id: UnorderedMap<ContributionId, Contribution>, //Link Contributions ID with Contribution
    pub metadata: LazyOption<ForMyFutureContractMetadata>, //Contract Metadata
}


//Keys for persistent collections
#[derive(BorshSerialize)]
pub enum StorageKey {
    ProposalsPerOwner,
    ProposalsById,
    ProposalMetadataById,
    ContributionsperUser,
    MyFutureContractMetadata,
    ContributionsById
}

#[near_bindgen]
impl Contract {


    #[init]
    pub fn new_meta(owner_id: AccountId) -> Self { //Method for initialize contract 
        
        Self::new(
            owner_id,
            ForMyFutureContractMetadata {
                name: "4MyFuture DApp".to_string()
            },
        )
    }
    
    #[init]
    pub fn new(owner_id: AccountId, metadata: ForMyFutureContractMetadata) -> Self { //Method called by new_meta for initialized all persistent collections
        let this = Self {
            owner_id: owner_id,
            proposal_per_owner: LookupMap::new(StorageKey::ProposalsPerOwner.try_to_vec().unwrap()),
            proposal_by_id: UnorderedMap::new(StorageKey::ProposalsById.try_to_vec().unwrap()),
            proposal_metadata_by_id: LookupMap::new(
                StorageKey::ProposalMetadataById.try_to_vec().unwrap(),
            ),
            contributions_per_user: LookupMap::new(
                StorageKey::ContributionsperUser.try_to_vec().unwrap(),
            ),
            contributions_per_id: UnorderedMap::new(StorageKey::ContributionsById.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::MyFutureContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}
