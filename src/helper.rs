use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::cell::RefCell;

pub(crate) fn create_pda_account<'a, W: BorshSerialize>(
    program_id: &Pubkey,
    signer: &AccountInfo<'a>,
    non_signer_as_seed: Option<Pubkey>,
    account_pda: &AccountInfo<'a>,
    bump: u8,
    extra_seed: &[u8],
    optional_seed: Option<&[u8]>,
    data: W,
) -> ProgramResult {
    let data_span = (borsh::to_vec(&data)?).len();
    let lamports_required = (Rent::get()?).minimum_balance(data_span);

    let seed_key = match non_signer_as_seed {
        Some(a) => a,
        None => signer.key.to_owned(),
    };

    if let Some(seed) = optional_seed {
        invoke_signed(
            &system_instruction::create_account(
                signer.key, // User is payer
                account_pda.key,
                lamports_required,
                data_span as u64,
                program_id,
            ),
            &[signer.clone(), account_pda.clone()],
            &[&[seed_key.as_ref(), extra_seed, seed, &[bump]]],
        )?;
    } else {
        invoke_signed(
            &system_instruction::create_account(
                signer.key, // User is payer
                account_pda.key,
                lamports_required,
                data_span as u64,
                program_id,
            ),
            &[signer.clone(), account_pda.clone()],
            &[&[seed_key.as_ref(), extra_seed, &[bump]]],
        )?;
    }

    data.serialize(&mut (&mut RefCell::borrow_mut(&account_pda.data)[..]))?;

    Ok(())
}

pub(crate) fn update_pda_account<'a, W: BorshSerialize>(
    signer: &AccountInfo<'a>,
    account_pda: &AccountInfo<'a>,
    data: W,
) -> ProgramResult {
    let data_span = (borsh::to_vec(&data)?).len();
    let lamports_required = (Rent::get()?).minimum_balance(data_span);

    if lamports_required > account_pda.lamports() {
        invoke(
            &system_instruction::transfer(
                signer.key,
                account_pda.key,
                lamports_required - account_pda.lamports(),
            ),
            &[signer.clone(), account_pda.clone()],
        )?;
    }

    account_pda.realloc((borsh::to_vec(&data)?).len(), false)?;

    data.serialize(&mut (&mut RefCell::borrow_mut(&account_pda.data)[..]))?;
    Ok(())
}
