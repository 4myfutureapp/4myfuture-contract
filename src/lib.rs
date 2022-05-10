use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault,
    Promise, CryptoHash, BorshStorageKey,
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
    pub metadata: LazyOption<ForMyFutureContractMetadata>

}