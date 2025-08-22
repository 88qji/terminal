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
