4MyFuture-Contract V2 🚀🎓
==================

This smart contract was created on Rust
4MyFuture DApp is a crowdfunding application made for students, in which they will have the oportunity to request fund
to the NEAR Ecosystem Community, explaining why they need the funds, uploading images, descriptions and much more 😎

Compile the Contract 💻
====================

Before you compile this code, you will need to install Rust with [correct target]
For compile the code just run the following script: 
`sh scripts/build.sh`
It will generate the .wasm file and will deploy it on testnet 


Exploring The Code 😎
==================

The main smart contract code lives in `src` folder.
Here you will find five .rs files:

1. Enumeration --> Here you will find all the contract query methods
2. Internal --> File created for manage internal complex methods that are called for the main methods
3. Lib --> Main file that initialize the contract and all persistent storage collections 
4. Metadata --> Here all the main contract structures live
5. Proposal --> The main functions folder

Project Links 👾
=================
1. Frontend [here](https://www.figma.com/file/NhF1w60RMd5qoHvH41Zdcc/4MyFuture?node-id=0%3A1)
2. Graph [here](https://thegraph.com/hosted-service/subgraph/edwardsvo/for-my-future)
3. Query the Graph [here](https://api.thegraph.com/subgraphs/name/edwardsvo/for-my-future)
4. Graph implementation Repo [here](https://github.com/EdwardsVO/4myfuture-graph)
5. DAO for test upgrade contract [here](https://testnet-v2.sputnik.fund/#/migrationtest1.sputnikv2.testnet)

Interact with the Contract 👽
=============================
# Call functions

1. create-proposal <--- Method for create proposal and receive funds
`near call ${CONTRACT-NAME} create_proposal '{"title":"test", "description": "test", "finish_date": 123321332342345, "images":["link1.affd.com", "link2.aaa.com"], "goal":"3", "institution_link":"www.unimet.com", "pensum_link":"www.unimet.com/ingenieria-sistemas"}' --account-id ${YOUR-ACCOUNT.testnet}`

2. contribute <--- Method for fund a proposal
`near call ${CONTRACT-NAME} contribute '{"proposal_id":"1"}' --account-id ${YOUR-ACCOUNT.testnet} --deposit 0.1`

# View Methods

1. proposals <--- Get all proposals registered
`near call ${CONTRACT-NAME} proposals`

2. proposals_by_id <--- Get a range of proposals registered
`near view ${CONTRACT-NAME} proposals_by_id '{"from_index":"0" , "limit":1}'`

3. proposal_by_id <--- Get a specified proposal by id
`near view ${CONTRACT-NAME} proposal_by_id '{"proposal_id":"1}'`

4. proposal_by_owner <--- Get the proposal created by an user
`near view  ${CONTRACT_NAME} proposal_by_owner '{"owner_id": "4my2.lexdev.testnet"}'`

5. contributions <--- Get all contributions registered
`near call ${CONTRACT-NAME} contributions`

6. contributions_by_id <--- Get a range of contributions
`near view ${CONTRACT-NAME} contributions_by_id '{"from_index":"0" , "limit":1}'`

7. contribution_by_id <--- Get a specified contribution by id
`near view ${CONTRACT-NAME} contribution_by_id '{"contribution_id":"1}'`

8. contribution_for_user <--- Get the contributions made for an user
`near view ${CONTRACT-NAME} contributions_for_user '{"account_id":"lexdev.testnet", "from_index":"0" , "limit":10}'`

CONTRACT_NAME example [dev-1652466503853-68305726825054](https://explorer.testnet.near.org/accounts/dev-1652466503853-68305726825054)


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
