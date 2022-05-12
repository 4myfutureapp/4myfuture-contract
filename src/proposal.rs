use crate::*;

#[near_bindgen]
impl Contract {
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        goal: U128,
        institution_link: String,
        pensum_link: String,
        finish_date: Date,
        images: Vec<String>,
    ) {
        assert!(goal.0 > 0, "Invalid amount provided");
        let user_id = env::signer_account_id().to_string();
        assert!(
            !self.user_with_active_proposal(user_id),
            "User already have one active proposal"
        );

        let goal_in_yocto = U128(goal.0 * ONE_NEAR);
        let index = U128((self.proposal_by_id.len() + 1) as u128);
        let initial_storage_usage = env::storage_usage();

        let proposal_metadata = ProposalMetadata {
            title: title,
            description: description,
            goal: goal_in_yocto,
            init_date: env::block_timestamp(),
            finish_date: finish_date,
            funds: 0,
            images: images.clone(),
            institution_link: institution_link,
            pensum_link: pensum_link,
        };

        let proposal = Proposal {
            id: index,
            owner: env::signer_account_id().to_string(),
            metadata: proposal_metadata.clone(),
            image: images[0].clone(),
            status: 0,
        };

        self.add_proposal_to_storages(proposal.clone(), env::signer_account_id());
        env::log(
            json!({
                "type": "create_proposal",
                "params":{
                    "proposal_id":proposal.id.0.to_string(),
                    "owner": proposal.owner
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    #[payable]
    pub fn contribute(&mut self, proposal_id: ProposalId) {
        assert!(env::attached_deposit() > 0 as u128, "Invalid contribution amount");
        assert!(self.proposal_by_id.get(&proposal_id).is_some(), "Invalid proposal id");
        let proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        assert!(proposal.owner != env::signer_account_id(), "You can't contribute your own proposal");
        self.process_contribution(env::attached_deposit(), proposal);
    }
}
