use crate::*;

#[near_bindgen]
impl NewContract {
    //Main contract function for create proposals
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
        assert!(goal.0 > 0, "Invalid amount provided"); //Goal required need to be higher than zero
        let user_id = env::signer_account_id().to_string();
        assert!(
            !self.user_with_active_proposal(user_id), //Check if user have an active proposal
            "User already have one active proposal"
        );

        let goal_in_yocto = U128(goal.0 * ONE_NEAR); //Parse from NEAR to Yocto
        let index = U128((self.proposal_by_id.len() + 1) as u128);
        let initial_storage_usage = env::storage_usage();

        let proposal_metadata = ProposalMetadata {
            //Create the proposal metadata
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
            //Inser the proposal metadata into Proposal object
            id: index,
            owner: env::signer_account_id().to_string(),
            metadata: proposal_metadata.clone(),
            image: images[0].clone(),
            status: 0,
        };

        self.add_proposal_to_storages(proposal.clone(), env::signer_account_id()); //Update the collections
        env::log(
            json!({
                    "id":proposal.id.0.to_string(),
                    "owner": proposal.owner.to_string(),
                    "status": proposal.status,
                    "image": proposal.image.to_string(),
            })
            .to_string()
            .as_bytes(),
        );
        env::log(
            json!({
                    "id": proposal.id.0.to_string(),
                    "title": proposal_metadata.title.to_string(),
                    "description": proposal_metadata.description.to_string(),
                    "goal": proposal_metadata.goal.0 as i64,
                    "init_date": proposal_metadata.init_date,
                    "finish_date": proposal_metadata.finish_date,
                    "funds": proposal_metadata.funds as i64,
                    "images": proposal_metadata.images,
                    "institution_link": proposal_metadata.institution_link.to_string(),
                    "pensum_link": proposal_metadata.pensum_link.to_string()
            })
            .to_string()
            .as_bytes(),
        )
    }

    //Inactive proposal and disable funding option
    pub fn inactive_proposal(&mut self, proposal_id: ProposalId) {
        //Inactive proposal from owner id
        assert!(
            env::signer_account_id() == self.owner_id,
            "Only owner can call this function"
        );
        self.update_proposal(proposal_id, 2); //Update the proposal status to "Inactive"
    }

    //Contribute function
    #[payable]
    pub fn contribute(&mut self, proposal_id: ProposalId) {
        assert!(
            self.proposal_by_id.get(&proposal_id).is_some(),
            "Invalid proposal id"
        ); //Check if proposal exist
        let proposal = self.proposal_by_id.get(&proposal_id).unwrap();
        assert!(proposal.status == 0, "Proposal not active"); //Check if proposal is able to receive funding
        self.valid_contribution_amount(proposal.clone(), env::attached_deposit()); //Check if the contribution is valid
        self.process_contribution(env::attached_deposit(), proposal.clone()); //Function for process the contribution and Log the transaction
        let proposa_meta = self.proposal_metadata_by_id.get(&proposal.id).unwrap();
        if proposa_meta.goal.0 == proposa_meta.funds {
            //Check if proposal goal is reached
            self.update_proposal(proposal.id, 1); //1 for status "Complete"
        }
    }
}
