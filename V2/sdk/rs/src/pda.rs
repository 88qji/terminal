use squads_multisig_program::{
    SEED_EPHEMERAL_SIGNER, SEED_MULTISIG, SEED_PREFIX, SEED_PROGRAM_CONFIG, SEED_PROPOSAL,
    SEED_SPENDING_LIMIT, SEED_TRANSACTION, SEED_VAULT,
};

use crate::solana_program::pubkey::Pubkey;

pub fn get_program_config_pda(program_id: Option<&Pubkey>) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[SEED_PREFIX, SEED_PROGRAM_CONFIG],
        program_id.unwrap_or(&squads_multisig_program::ID),
    )
}

pub fn get_multisig_pda(create_key: &Pubkey, program_id: Option<&Pubkey>) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[SEED_PREFIX, SEED_MULTISIG, create_key.to_bytes().as_ref()],
        program_id.unwrap_or(&squads_multisig_program::ID),
    )
}

