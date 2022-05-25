use crate::*;
use near_sdk::CryptoHash;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

#[near_bindgen]
impl Contract {
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

    pub(crate) fn add_proposal_to_storages(&mut self, proposal: Proposal, proposal_metadata: ProposalMetadata, account_id: AccountId) {
        //Update the contract proposal collections with new data
        self.proposal_by_id.insert(&proposal.id, &proposal);
        self.proposal_metadata_by_id
            .insert(&proposal.id, &proposal_metadata);
        self.proposal_per_owner.insert(&account_id, &proposal);
    }

    pub(crate) fn update_proposal(&mut self, proposal_id: ProposalId, status: i8) {
        //status 1: complete ---- status 2: Inactive
        let mut proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        proposal.status = status;
        let proposal_metadata = self.proposal_metadata_by_id.get(&proposal_id).unwrap();
        self.add_proposal_to_storages(proposal.clone(), proposal_metadata, proposal.owner.clone());
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

    pub(crate) fn process_contribution(&mut self, amount: Balance, proposal: Proposal) {
        let user = env::signer_account_id();
        let mut proposal_meta = self.proposal_metadata_by_id.get(&proposal.id).unwrap();
        Promise::new(proposal.owner.clone()).transfer(amount);
        proposal_meta.funds += amount;
        self.add_proposal_to_storages(proposal.clone(), proposal_meta.clone(), proposal.owner.clone()); //Update the proposal funds attribute

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
        let proposal_meta = self.proposal_metadata_by_id.get(&proposal.id).unwrap();
        let current_funds = contribution_amount + proposal_meta.funds;
        assert!(
            current_funds <= proposal_meta.goal.0,
            "Contribution higher than expected"
        );
    }
}
