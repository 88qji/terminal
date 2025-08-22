// Re-export dependencies for convenience
pub use solana_client;

pub use squads_multisig_program;
pub use squads_multisig_program::anchor_lang;
pub use squads_multisig_program::anchor_lang::solana_program;

pub mod client;
pub mod pda;
pub mod vault_transaction;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ClientError {
        #[error(transparent)]
        Client(#[from] solana_client::client_error::ClientError),
        #[error("Failed to deserialize account data")]
        DeserializationError,
        #[error("Invalid AddressLookupTableAccount")]
        InvalidAddressLookupTableAccount,
        #[error("Invalid TransactionMessage")]
        InvalidTransactionMessage,
    }
}
