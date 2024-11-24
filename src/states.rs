use std::collections::HashMap;

use crate::states;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
pub(crate) enum Permission {
    Initiate {},
    Vote {},
    Execute {},
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct MultiSig {
    pub(crate) bump: u8,
    pub(crate) creator: Pubkey,
    pub(crate) signers: HashMap<Pubkey, Vec<Permission>>,
    pub(crate) minimum_number_of_signs: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct MultiSigVault {
    pub(crate) bump: u8,
    pub(crate) creator: Pubkey,
    pub(crate) note: String,
    pub(crate) data: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct InProcessMultiSig {
    pub(crate) bump: u8,
    pub(crate) creator: Pubkey,
    pub(crate) actions: Vec<String>, // Prefer uuid
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) enum Action {
    // MultiSig
    UpdateSigners {
        signers: HashMap<Pubkey, Vec<states::Permission>>,
    },
    UpdateMinimumNumberOfSigns {
        value: u32,
    },

    UpdateNote {
        note: String,
    },
    UpdateData {
        data: Vec<u8>,
    },

    // Common
    DeleteMultisig {},
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct MultiSigAction {
    pub(crate) bump: u8,
    pub(crate) action_id: String, // Use in PDA
    pub(crate) creator: Pubkey,
    pub(crate) action: Action,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct MultiSigVoting {
    pub(crate) bump: u8,
    pub(crate) action_id: String, // Use in PDA
    pub(crate) vote_by_signers: HashMap<Pubkey, Option<bool>>,
}
