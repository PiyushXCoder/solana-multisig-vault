use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use std::collections::HashMap;

use crate::{helper, states};

pub(crate) fn init_multisig_action(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    action_id: String,
    action: states::Action,
    multisig_action_account_bump: u8,
    multisig_voting_account_bump: u8,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let creator = next_account_info(account_iter)?;
    let multisig_action_account_pda = next_account_info(account_iter)?;
    let multisig_voting_account_pda = next_account_info(account_iter)?;
    let in_progress_multisig_account_pda = next_account_info(account_iter)?;
    let multisig_account_pda = next_account_info(account_iter)?;

    let multisig = states::MultiSig::try_from_slice(&multisig_account_pda.data.borrow())?;

    if let Some(perm) = multisig.signers.get(creator.key) {
        if !perm.contains(&states::Permission::Initiate {}) {
            panic!("Unauthorized");
        }
    } else {
        panic!("Unauthorized");
    }

    // For Action Account
    let multisig_action = states::MultiSigAction {
        bump: multisig_action_account_bump,
        action_id: action_id.clone(),
        action,
    };

    helper::create_pda_account(
        program_id,
        &creator,
        None,
        &multisig_action_account_pda,
        multisig_action_account_bump,
        b"action",
        Some(action_id.as_bytes()),
        multisig_action,
    )?;

    // For Voting Account
    let mut vote_by_signers: HashMap<Pubkey, Option<bool>> = HashMap::new();
    vote_by_signers.insert(creator.key.to_owned(), None);

    for account in account_iter {
        vote_by_signers.insert(account.key.to_owned(), None);
    }

    let multisig_voting = states::MultiSigVoting {
        bump: multisig_voting_account_bump,
        action_id: action_id.clone(),
        vote_by_signers,
    };

    helper::create_pda_account(
        program_id,
        &creator,
        None,
        &multisig_voting_account_pda,
        multisig_voting_account_bump,
        b"voting",
        Some(action_id.as_bytes()),
        multisig_voting,
    )?;

    let mut in_progress_multisig =
        states::InProcessMultiSig::try_from_slice(&in_progress_multisig_account_pda.data.borrow())?;
    if in_progress_multisig.actions.contains(&action_id) {
        panic!("In Progress Exists");
    }
    in_progress_multisig.actions.push(action_id);

    helper::update_pda_account(
        &creator,
        &in_progress_multisig_account_pda,
        in_progress_multisig,
    )?;

    Ok(())
}
