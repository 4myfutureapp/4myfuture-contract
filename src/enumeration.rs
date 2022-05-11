use crate::*;

#[near_bindgen]
impl Contract {
    pub fn proposal_total_supply(&self) -> U128 {
        return U128(self.proposal_by_id.len() as u128);
    }

    pub fn get_proposals(&self) -> Vec<Proposal> {
        return self.proposal_by_id.values_as_vector().to_vec();
    }
}
