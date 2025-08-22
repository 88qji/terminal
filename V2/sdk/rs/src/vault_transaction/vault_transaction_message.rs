use crate::pda::get_ephemeral_signer_pda;
use squads_multisig_program::{
    CompiledInstruction, MessageAddressTableLookup, TransactionMessage, VaultTransactionMessage,
};
use std::collections::HashMap;

use super::compiled_keys::CompiledKeys;
use crate::solana_program::address_lookup_table_account::AddressLookupTableAccount;
use crate::solana_program::instruction::{AccountMeta, Instruction};
use crate::solana_program::message::{AccountKeys, CompileError};
use crate::solana_program::pubkey::Pubkey;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid AddressLookupTableAccount")]
    InvalidAddressLookupTableAccount,
    #[error("Invalid TransactionMessage")]
    InvalidTransactionMessage,
}
