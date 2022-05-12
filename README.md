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



  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
