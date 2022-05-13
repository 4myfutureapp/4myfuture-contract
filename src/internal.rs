use crate::*;
use near_sdk::CryptoHash;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

#[near_bindgen]
impl NewContract {
    pub(crate) fn user_with_active_proposal(&self, account_id: AccountId) -> bool {
        //Check if user has an active proposal (status == 0)
        if !self.proposal_per_owner.contains_key(&account_id) {
            return false;
        }
        let proposal = self.proposal_per_owner.get(&account_id).unwrap();
        if proposal.status == 0 {
            return true;
        }
        false
    }

    pub(crate) fn add_proposal_to_storages(&mut self, proposal: Proposal, account_id: AccountId) {
        //Update the contract proposal collections with new data
        self.proposal_by_id.insert(&proposal.id, &proposal);
        self.proposal_metadata_by_id
            .insert(&proposal.id, &proposal.metadata);
        self.proposal_per_owner.insert(&account_id, &proposal);
    }

    pub(crate) fn update_proposal(&mut self, proposal_id: ProposalId, status: i8) {
        //status 1: complete ---- status 2: Inactive
        let mut proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        proposal.status = status;
        self.add_proposal_to_storages(proposal.clone(), proposal.owner.clone());
        env::log(
            json!({
                    "id": proposal.id.0.to_string(),
                    "owner": proposal.owner.to_string(),
                    "status": proposal.status.to_string()
            })
            .to_string()
            .as_bytes(),
        );
    }

    pub(crate) fn process_contribution(&mut self, amount: Balance, mut proposal: Proposal) {
        let user = env::signer_account_id();
        let mut proposal_meta = self.proposal_metadata_by_id.get(&proposal.id).unwrap();
        Promise::new(proposal.owner.clone()).transfer(amount);
        proposal.metadata.funds += amount;
        proposal_meta.funds += amount;
        self.add_proposal_to_storages(proposal.clone(), proposal.owner.clone()); //Update the proposal funds attribute

        let index = U128(self.contributions_per_id.len() as u128);
        let contribution = Contribution {
            // Create the contribution object
            id: index,
            from: user.clone(),
            to: proposal.owner.clone(),
            proposal_id: proposal.id,
            amount: env::attached_deposit(),
            image: proposal_meta.images[0].to_string(),
        };
        self.contributions_per_id.insert(&index, &contribution); //Insert contribution in collection
        let mut contributions_set = self.contributions_per_user.get(&user).unwrap_or_else(|| {
            //Check if user already has contributions made, if not it will create an empty collection
            UnorderedSet::new(
                StorageKey::ContributionsPerUserInner {
                    account_id_hash: hash_account_id(&proposal.owner), //acount id hashed into collection as key for avoid collisions
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        contributions_set.insert(&contribution); //Insert the contribution object within the user
        self.contributions_per_user
            .insert(&user, &contributions_set);
        env::log(
            json!({
                    "id": contribution.id.0.to_string(),
                    "from": contribution.from.to_string(),
                    "to": contribution.to.to_string(),
                    "proposal_id": contribution.proposal_id.0.to_string(),
                    "amount": contribution.amount as i64,
                    "image": contribution.image.to_string()
            })
            .to_string()
            .as_bytes(),
        );
    }

    pub(crate) fn valid_contribution_amount(
        &self,
        proposal: Proposal,
        contribution_amount: Balance,
    ) {
        assert!(contribution_amount > 0, "invalid contribution amount");
        assert!(
            proposal.owner != env::signer_account_id(),
            "Can't contribute your own proposal"
        );
        let current_funds = contribution_amount + proposal.metadata.funds;
        assert!(
            current_funds <= proposal.metadata.goal.0,
            "Contribution higher than expected"
        );
    }
}
