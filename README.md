# Solana multisig vault

It is a multi sig vault made from scratch using solana blockchain. It uses solana program and pda to restrict write permission on data. 

> Note: Data can be read without permission. 

## How it is different from tradition encryption based multisig approaches?

Since any action(update) to data in the vault is being written in blockchain, we have traceability. In case of corruption the history of who participated remains in the blockchain.

## How I can try?

It is simple to build and execute. 

- Setup environment for solana native and build and deploy. [This guide would be helpful.](https://solana.com/docs/intro/installation)
- Use `cargo build-sbf` to build project
- Use `solana program deploy target/deploy/solana_multisig_vault.so` to deploy it. I recommend running local validator since it is easy to clean.
- You can try test scripts `PROG=<pubkey of program> KEYPAIR1=<path to key file 1> KEYPAIR2=<path to key file 2> bash test/test_all.sh`
  > We have used multiple key file to check for 2 votes. Read late sections to understand flow of multisig

## But I am lazy!

Try it on devnet. Fine?

```Program Id: ESJ5HdSAAxv9k94zfJNVtpQ2CjmVndyaKQrA9PQQ94Wt```

## What is the workflow?

So it works in 4 steps!

- First of all you create a multisig account. The account now belongs to you*. There are basically 3 PDA accounts created.
  - Multisg: Keep track of data necessary for permissions and voting rights
  - Vault: Keeps actual data
  - In Process: Keeps lits of id of actions active for your multisig
  > *Note: The PDA account is owned by program.
- Propose an action: Any pubkey mentioned in multisig account with `Initiate` can propose an action. Proposing an action created 2 new account for that action,
  - Action: Stores the action to be taken when voting finishes
  - Voting: Keeps track of votes' vote.
  > Note: Each proposal has its own seperate action and voting account
- Voting, as you can guess people votes the proposal
- Execution, The action is performed and the temproary accounts are cleaned

Thats all! Check test scripts to know more.

## Any TODO?

- [ ] Add command to delete unsed proposal


