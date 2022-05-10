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

use crate::metadata::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub proposal_per_owner: LookupMap<AccountId, String>,
    pub proposal_by_id: LookupMap<ProposalId, Proposal>,
    pub proposal_metadata_by_id: LookupMap<ProposalId, ProposalMetadata>,
    pub contributions_per_user: LookupMap<AccountId, Contribution>,
    pub metadata: LazyOption<ForMyFutureContractMetadata>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    ProposalsPerOwner,
    ProposalsById,
    ProposalMetadataById,
    ContributionsperUser,
    MyFutureContractMetadata
}

#[near_bindgen]
impl Contract {


    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        
        Self::new(
            owner_id,
            ForMyFutureContractMetadata {
                name: "4MyFuture DApp".to_string()
            },
        )
    }
    
    #[init]
    pub fn new(owner_id: AccountId, metadata: ForMyFutureContractMetadata) -> Self {
        let this = Self {
            owner_id: owner_id,
            proposal_per_owner: LookupMap::new(StorageKey::ProposalsPerOwner.try_to_vec().unwrap()),
            proposal_by_id: LookupMap::new(StorageKey::ProposalsById.try_to_vec().unwrap()),
            proposal_metadata_by_id: LookupMap::new(
                StorageKey::ProposalMetadataById.try_to_vec().unwrap(),
            ),
            contributions_per_user: LookupMap::new(
                StorageKey::ContributionsperUser.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::MyFutureContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}
