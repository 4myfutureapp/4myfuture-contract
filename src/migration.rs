use crate::*;

#[near_bindgen]
impl NewContract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_contract_state: OldContract = env::state_read().expect("failed");
        Self {
           contributions_per_id: old_contract_state.contributions_per_id,
           contributions_per_user: old_contract_state.contributions_per_user,
           metadata: old_contract_state.metadata,
           proposal_by_id: old_contract_state.proposal_by_id,
           owner_id:  old_contract_state.owner_id,
           proposal_metadata_by_id: old_contract_state.proposal_metadata_by_id,
           proposal_per_owner: old_contract_state.proposal_per_owner,
           proposal_from_migration: LookupMap::new(b"a".to_vec())
        }
    }
}
