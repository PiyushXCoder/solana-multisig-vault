use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
};

use crate::{helper, states};

pub(crate) fn vote_multisig_action(accounts: &[AccountInfo], vote: bool) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let voter = next_account_info(account_iter)?;
    let multisig_voting_account_pda = next_account_info(account_iter)?;
    let multisig_account_pda = next_account_info(account_iter)?;

    let multisig = states::MultiSig::try_from_slice(&multisig_account_pda.data.borrow())?;

    if let Some(perm) = multisig.signers.get(voter.key) {
        if !perm.contains(&states::Permission::Vote {}) {
            panic!("Unauthorized");
        }
    } else {
        panic!("Unauthorized");
    }

    let mut multising_voting =
        states::MultiSigVoting::try_from_slice(&multisig_voting_account_pda.data.borrow())?;

    if let Some(d) = multising_voting.vote_by_signers.get_mut(voter.key) {
        *d = Some(vote);
    } else {
        panic!("Voter not found");
    }

    helper::update_pda_account(&voter, &multisig_voting_account_pda, multising_voting)?;

    Ok(())
}
