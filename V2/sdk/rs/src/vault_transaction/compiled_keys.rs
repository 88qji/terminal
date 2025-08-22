use std::collections::BTreeMap;

use crate::solana_program::address_lookup_table_account::AddressLookupTableAccount;
use crate::solana_program::instruction::Instruction;
use crate::solana_program::message::v0::{LoadedAddresses, MessageAddressTableLookup};
use crate::solana_program::message::{CompileError, MessageHeader};

use crate::solana_program::pubkey::Pubkey;

/// A helper struct to collect pubkeys compiled for a set of instructions
///
/// NOTE: The only difference between this and the original implementation from `solana_program` is that we don't mark the instruction programIds as invoked.
// /// It makes sense to do because the instructions will be called via CPI, so the programIds can come from Address Lookup Tables.
// /// This allows to compress the message size and avoid hitting the tx size limit during `vault_transaction_create` instruction calls.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompiledKeys {
    payer: Option<Pubkey>,
    key_meta_map: BTreeMap<Pubkey, CompiledKeyMeta>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct CompiledKeyMeta {
    is_signer: bool,
    is_writable: bool,
    is_invoked: bool,
}

impl CompiledKeys {
    /// Compiles the pubkeys referenced by a list of instructions and organizes by
    /// signer/non-signer and writable/readonly.
    pub(crate) fn compile(instructions: &[Instruction], payer: Option<Pubkey>) -> Self {
        let mut key_meta_map = BTreeMap::<Pubkey, CompiledKeyMeta>::new();
        for ix in instructions {
            let meta = key_meta_map.entry(ix.program_id).or_default();
            // NOTE: This is the only difference from the original.
            // meta.is_invoked = true;
            meta.is_invoked = false;
            for account_meta in &ix.accounts {
                let meta = key_meta_map.entry(account_meta.pubkey).or_default();
                meta.is_signer |= account_meta.is_signer;
                meta.is_writable |= account_meta.is_writable;
            }
        }
        if let Some(payer) = &payer {
            let meta = key_meta_map.entry(*payer).or_default();
            meta.is_signer = true;
            meta.is_writable = true;
        }
        Self {
            payer,
            key_meta_map,
        }
    }
