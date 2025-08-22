use solana_client::nonblocking::rpc_client::RpcClient;

pub use squads_multisig_program::accounts::BatchAccountsClose as BatchAccountsCloseAccounts;
pub use squads_multisig_program::accounts::ConfigTransactionAccountsClose as ConfigTransactionAccountsCloseAccounts;
pub use squads_multisig_program::accounts::ConfigTransactionCreate as ConfigTransactionCreateAccounts;
pub use squads_multisig_program::accounts::ConfigTransactionExecute as ConfigTransactionExecuteAccounts;
pub use squads_multisig_program::accounts::MultisigCreateV2 as MultisigCreateAccountsV2;
pub use squads_multisig_program::accounts::ProposalCreate as ProposalCreateAccounts;
pub use squads_multisig_program::accounts::ProposalVote as ProposalVoteAccounts;
pub use squads_multisig_program::accounts::SpendingLimitUse as SpendingLimitUseAccounts;
pub use squads_multisig_program::accounts::VaultBatchTransactionAccountClose as VaultBatchTransactionAccountCloseAccounts;
pub use squads_multisig_program::accounts::VaultTransactionAccountsClose as VaultTransactionAccountsCloseAccounts;
pub use squads_multisig_program::accounts::VaultTransactionCreate as VaultTransactionCreateAccounts;
pub use squads_multisig_program::accounts::VaultTransactionExecute as VaultTransactionExecuteAccounts;
use squads_multisig_program::anchor_lang::AnchorSerialize;
pub use squads_multisig_program::instruction::ConfigTransactionAccountsClose as ConfigTransactionAccountsCloseData;
pub use squads_multisig_program::instruction::ConfigTransactionCreate as ConfigTransactionCreateData;
pub use squads_multisig_program::instruction::ConfigTransactionExecute as ConfigTransactionExecuteData;
pub use squads_multisig_program::instruction::MultisigCreate as MultisigCreateData;
pub use squads_multisig_program::instruction::MultisigCreateV2 as MultisigCreateDataV2;
pub use squads_multisig_program::instruction::ProposalApprove as ProposalApproveData;
pub use squads_multisig_program::instruction::ProposalCancel as ProposalCancelData;
pub use squads_multisig_program::instruction::ProposalCreate as ProposalCreateData;
pub use squads_multisig_program::instruction::SpendingLimitUse as SpendingLimitUseData;
pub use squads_multisig_program::instruction::VaultTransactionAccountsClose as VaultTransactionAccountsCloseData;
pub use squads_multisig_program::instruction::VaultTransactionCreate as VaultTransactionCreateData;
pub use squads_multisig_program::instruction::VaultTransactionExecute as VaultTransactionExecuteData;
pub use squads_multisig_program::instructions::ConfigTransactionCreateArgs;
pub use squads_multisig_program::instructions::MultisigCreateArgsV2;
pub use squads_multisig_program::instructions::ProposalCreateArgs;
pub use squads_multisig_program::instructions::ProposalVoteArgs;
pub use squads_multisig_program::instructions::SpendingLimitUseArgs;
pub use squads_multisig_program::instructions::VaultTransactionCreateArgs;
use squads_multisig_program::TransactionMessage;

use crate::anchor_lang::prelude::Pubkey;
use crate::anchor_lang::AccountDeserialize;
use crate::anchor_lang::{
    solana_program::instruction::Instruction, InstructionData, ToAccountMetas,
};
use crate::client::utils::IntoAccountMetas;
use crate::error::ClientError;
use crate::pda::get_vault_pda;
use crate::solana_program::address_lookup_table_account::AddressLookupTableAccount;
use crate::solana_program::instruction::AccountMeta;
use crate::state::{Multisig, SpendingLimit};
use crate::vault_transaction::{Error, VaultTransactionMessageExt};
use crate::ClientResult;

/// Gets a `Multisig` account from the chain.
pub async fn get_multisig(rpc_client: &RpcClient, multisig_key: &Pubkey) -> ClientResult<Multisig> {
    let multisig_account = rpc_client.get_account(multisig_key).await?;

    let multisig = Multisig::try_deserialize(&mut multisig_account.data.as_slice())
        .map_err(|_| ClientError::DeserializationError)?;

    Ok(multisig)
}
