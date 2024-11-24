pub(crate) mod helper;
pub(crate) mod instructions;
pub(crate) mod states;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {

    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
        pubkey::Pubkey,
    };

    use crate::{instructions, states};

    solana_program::entrypoint!(process_instruction);

    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let command = RequestData::try_from_slice(_instruction_data);

        #[allow(unused)]
        match command {
            Err(e) => {
                return Err(ProgramError::InvalidArgument);
                panic!("{:?}", e);
            }
            Ok(RequestData::CreateMultiSig {
                permissions,
                minimum_number_of_signs_for_update,
                note,
                multisig_account_bump,
                multisig_vault_account_bump,
                in_progress_multisig_account_bump,
            }) => {
                instructions::create_card_account(
                    _program_id,
                    _accounts,
                    permissions,
                    minimum_number_of_signs_for_update,
                    note,
                    multisig_account_bump,
                    multisig_vault_account_bump,
                    in_progress_multisig_account_bump,
                )?;
            }
            Ok(RequestData::ExecuteMultiSigAction {}) => {
                instructions::execute_multisig_action(_accounts)?;
            }
            Ok(RequestData::InitMultiSigAction {
                action_id,
                action,
                multisig_action_account_bump,
                multisig_voting_account_bump,
            }) => {
                instructions::init_multisig_action(
                    _program_id,
                    _accounts,
                    action_id,
                    action,
                    multisig_action_account_bump,
                    multisig_voting_account_bump,
                )?;
            }
            Ok(RequestData::VoteMultiSigAction { vote }) => {
                instructions::vote_multisig_action(_accounts, vote)?;
            }
            _ => {
                panic!("Bad request")
            }
        }

        Ok(())
    }

    #[derive(BorshSerialize, BorshDeserialize, Debug)]
    enum RequestData {
        // creator, multisig_account_pda, multisig_vault_account_pda
        CreateMultiSig {
            permissions: Vec<Vec<states::Permission>>,
            minimum_number_of_signs_for_update: u32,
            note: String,
            multisig_account_bump: u8,
            multisig_vault_account_bump: u8,
            in_progress_multisig_account_bump: u8,
        },

        // executor, multisig_action_account_pda, multisig_voting_account_pda,
        // multisig_account_pda, multisig_vault_account_pda, in_progress_multisig_account_pda
        ExecuteMultiSigAction {},

        // creator, multisig_action_account_pda, multisig_voting_account_pda,
        // in_progress_multisig_account_pda, multisig_account_pda
        InitMultiSigAction {
            action_id: String,
            action: states::Action,
            multisig_action_account_bump: u8,
            multisig_voting_account_bump: u8,
        },

        // voter, multisig_voting_account_pda,multisig_account_pda
        VoteMultiSigAction {
            vote: bool,
        },
    }
}
