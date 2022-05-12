use crate::*;

#[near_bindgen]
impl Contract {
    pub fn proposal_total_supply(&self) -> U128 {
        return U128(self.proposal_by_id.len() as u128);
    }

    pub fn proposals(&self) -> Vec<Proposal> {
        return self.proposal_by_id.values_as_vector().to_vec();
    }

    pub fn contributions(&self) -> Vec<Contribution> {
        return self.contributions_per_id.values_as_vector().to_vec();
    }

    pub fn proposal_by_id(&self, proposal_id: ProposalId) -> Option<Proposal> {
        let proposal = self.proposal_by_id.get(&proposal_id);
        return proposal
    }

    pub fn contribution_by_id(&self, contribution_id: ContributionId) -> Option<Contribution> {
        let contribution = self.contributions_per_id.get(&contribution_id);
        return contribution
    }

    pub fn proposal_by_owner(&self, owner_id: AccountId) -> Option<Proposal> {
        let proposal = self.proposal_per_owner.get(&owner_id);
        return proposal;
    }

     
     pub fn proposals_by_id(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Proposal> {
        
        let start = u128::from(from_index.unwrap_or(U128(0)));

        
        self.proposal_by_id.keys()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|proposal_id| self.proposal_by_id(proposal_id.clone()).unwrap())
            .collect()
    }

    pub fn contributions_by_id(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Contribution> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        
       return self.contributions_per_id.keys()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|contribution_by_id| self.contribution_by_id(contribution_by_id.clone()).unwrap())
            .collect();
    }

    pub fn contributions_for_user(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Contribution> {
   
        let contributions_per_user = self.contributions_per_user.get(&account_id);
        let contributions = if let Some(contributions_per_user) = contributions_per_user  {
            contributions_per_user 
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));
        contributions.iter()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|contribution_id| self.contribution_by_id(contribution_id.id.clone()).unwrap())
            .collect()
    }

    pub fn contributions_supply_user (&self, account_id: AccountId) -> U128 {
        let cont = self.contributions_per_user.get(&account_id);
        if let Some(cont) = cont {
            U128(cont.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }
}
