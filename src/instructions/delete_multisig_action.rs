use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
};

use crate::{helper, states};

pub(crate) fn delete_multisig_action(accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let creator = next_account_info(account_iter)?;
    let multisig_action_account_pda = next_account_info(account_iter)?;
    let multisig_voting_account_pda = next_account_info(account_iter)?;
    let in_progress_multisig_account_pda = next_account_info(account_iter)?;

    let multisig_action =
        states::MultiSigAction::try_from_slice(&multisig_action_account_pda.data.borrow())?;
    let mut in_progress_multisig =
        states::InProcessMultiSig::try_from_slice(&in_progress_multisig_account_pda.data.borrow())?;
    let multisig_voting =
        states::MultiSigVoting::try_from_slice(&multisig_voting_account_pda.data.borrow())?;
    states::InProcessMultiSig::try_from_slice(&in_progress_multisig_account_pda.data.borrow())?;

    if &multisig_action.creator != creator.key || !creator.is_signer {
        panic!("Badly Signed!");
    }

    if !in_progress_multisig
        .actions
        .contains(&multisig_action.action_id)
    {
        panic!("Improper multisig")
    }

    if multisig_voting.action_id != multisig_action.action_id {
        panic!("action and voting pda mismatch")
    }

    in_progress_multisig
        .actions
        .retain(|x| *x != multisig_action.action_id);

    helper::update_pda_account(
        &creator,
        &in_progress_multisig_account_pda,
        in_progress_multisig,
    )?;

    let lamports = multisig_action_account_pda.lamports();
    **multisig_action_account_pda.try_borrow_mut_lamports()? -= lamports;
    **creator.try_borrow_mut_lamports()? += lamports;

    let lamports = multisig_voting_account_pda.lamports();
    **multisig_voting_account_pda.try_borrow_mut_lamports()? -= lamports;
    **creator.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
