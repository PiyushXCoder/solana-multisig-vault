use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use std::collections::HashMap;

use crate::{helper, states};

pub(crate) fn create_card_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    permissions: Vec<Vec<states::Permission>>,
    minimum_number_of_signs: u32,
    note: String,
    multisig_account_bump: u8,
    multisig_vault_account_bump: u8,
    in_progress_multisig_account_bump: u8,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let creator = next_account_info(account_iter)?;
    let multisig_account_pda = next_account_info(account_iter)?;
    let multisig_vault_account_pda = next_account_info(account_iter)?;
    let in_progress_multisig_account_pda = next_account_info(account_iter)?;

    if !creator.is_signer {
        panic!("Badly Signed!");
    }

    // For MultiSig Account
    let mut signers: HashMap<Pubkey, Vec<states::Permission>> = HashMap::new();
    signers.insert(
        creator.key.to_owned(),
        vec![
            states::Permission::Initiate {},
            states::Permission::Vote {},
            states::Permission::Execute {},
        ],
    );

    for (account, permission) in account_iter.zip(permissions.iter()) {
        signers.insert(account.key.to_owned(), permission.to_owned());
    }

    let multisig = states::MultiSig {
        bump: multisig_account_bump,
        creator: creator.key.to_owned(),
        signers,
        minimum_number_of_signs,
    };

    helper::create_pda_account(
        program_id,
        &creator,
        None,
        &multisig_account_pda,
        multisig_account_bump,
        b"multisig",
        None,
        multisig,
    )?;

    // For Multisig Vault Account
    let multisig_vault = states::MultiSigVault {
        bump: multisig_vault_account_bump,
        creator: creator.key.to_owned(),
        note,
        data: Vec::new(),
    };

    helper::create_pda_account(
        program_id,
        &creator,
        None,
        &multisig_vault_account_pda,
        multisig_vault_account_bump,
        b"vault",
        None,
        multisig_vault,
    )?;

    // In progress
    let in_progress_multisig = states::InProcessMultiSig {
        bump: in_progress_multisig_account_bump,
        creator: creator.key.to_owned(),
        actions: vec![],
    };

    helper::create_pda_account(
        program_id,
        &creator,
        None,
        &in_progress_multisig_account_pda,
        in_progress_multisig_account_bump,
        b"progress",
        None,
        in_progress_multisig,
    )?;

    Ok(())
}
