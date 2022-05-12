use crate::*;
use near_sdk::{CryptoHash};

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

    pub(crate) fn set_update_proposal(&mut self, proposal_id: ProposalId, status: i8) {
        let mut proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        proposal.status = status;
        self.proposal_by_id.insert(&proposal_id, &proposal);
    }

    pub(crate) fn process_contribution(&mut self, amount: Balance, mut proposal: Proposal) {

        let mut proposal_meta = self.proposal_metadata_by_id.get(&proposal.id).unwrap();
        let mut hash = CryptoHash::default();
        hash.copy_from_slice(&env::sha256(proposal.owner.as_bytes().clone()));
        Promise::new(proposal.owner.clone()).transfer(amount);
        proposal.metadata.funds += amount;
        proposal_meta.funds += amount;
        self.proposal_by_id.insert(&proposal.id, &proposal);
        self.proposal_metadata_by_id.insert(&proposal.id, &proposal_meta);

        let index = U128(self.contributions_per_id.len() as u128);
        let contribution = Contribution {
            id: index,
            from: env::signer_account_id(),
            to: proposal.owner.clone(),
            proposal_id: proposal.id,
            amount: env::attached_deposit(),
            image: proposal_meta.images[0].to_string()
        };
        self.contributions_per_id.insert(&index, &contribution);

        let mut contributions_set = self.contributions_per_user.get(&env::signer_account_id()).unwrap_or_else(|| {
            
            UnorderedSet::new(
                StorageKey::ContributionsPerUserInner {
                    account_id_hash: hash
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        contributions_set.insert(&contribution);
        self.contributions_per_user.insert(&proposal.owner, &contributions_set);
        env::log(
            json!({
                "type": "create_contribution",
                "params":{
                    "id": contribution.id.0.to_string(),
                    "from": contribution.from,
                    "to": contribution.to,
                    "proposal_id": contribution.proposal_id.0.to_string(),
                    "amount": contribution.amount.to_string()
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

        pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
            //get the default hash
            let mut hash = CryptoHash::default();
            //we hash the account ID and return it
            hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
            hash
        }
    }

