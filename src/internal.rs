use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn user_with_active_proposal(&self, account_id: AccountId) -> bool {
        if !self.proposal_per_owner.contains_key(&account_id) {
            return false 
        }
        let proposal = self.proposal_per_owner.get(&account_id).unwrap();
        if proposal.status == 0 {
            return true
        }
        false
    }

    pub(crate) fn add_proposal_to_storages(&mut self, proposal: Proposal, account_id: AccountId) {
        self.proposal_by_id.insert(&proposal.id, &proposal);
        self.proposal_metadata_by_id.insert(&proposal.id, &proposal.metadata);
        self.proposal_per_owner.insert(&account_id, &proposal);
    }

    pub(crate) fn set_inactive_proposal(&mut self, proposal_id: ProposalId) {
        let mut proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        proposal.status = 0;
        self.proposal_by_id.insert(&proposal_id, &proposal);
    }

    pub(crate) fn set_complete_proposal(&mut self, proposal_id: ProposalId) {
        let mut proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        proposal.status = 2;
        self.proposal_by_id.insert(&proposal_id, &proposal);
    }
}
