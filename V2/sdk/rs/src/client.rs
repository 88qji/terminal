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

/// Gets a `SpendingLimit` account from the chain.
pub async fn get_spending_limit(
    rpc_client: &RpcClient,
    spending_limit_key: &Pubkey,
) -> ClientResult<SpendingLimit> {
    let spending_limit_account = rpc_client.get_account(spending_limit_key).await?;

    let spending_limit =
        SpendingLimit::try_deserialize(&mut spending_limit_account.data.as_slice())
            .map_err(|_| ClientError::DeserializationError)?;

    Ok(spending_limit)
}

/// Creates a new multisig config transaction.
/// Example:
/// ```
/// use squads_multisig::anchor_lang::error::ComparedValues::Pubkeys;
/// use squads_multisig::solana_program::pubkey::Pubkey;
/// use squads_multisig::solana_program::system_program;
/// use squads_multisig::state::{ConfigAction, Member, Permissions, Permission};
/// use squads_multisig::client::{
///     MultisigCreateAccountsV2,
///     MultisigCreateArgsV2,
///     multisig_create_v2
/// };
///
/// let ix = multisig_create_v2(
///     MultisigCreateAccountsV2 {
///         program_config: Pubkey::new_unique(),
///         treasury: Pubkey::new_unique(),
///         multisig: Pubkey::new_unique(),
///         create_key: Pubkey::new_unique(),
///         creator: Pubkey::new_unique(),
///         system_program: system_program::id(),
///     },
///     MultisigCreateArgsV2 {
///         members: vec![
///             Member {
///                 key: Pubkey::new_unique(),
///                 permissions: Permissions::from_vec(&[Permission::Initiate, Permission::Vote, Permission::Execute]),
///             }
///         ],
///         threshold: 1,
///         time_lock: 0,
///         config_authority: None,
///         rent_collector: None,
///         memo: Some("Deploy my own Squad".to_string()),
///     },
///     Some(squads_multisig_program::ID)
/// );
/// ```
///
pub fn multisig_create_v2(
    accounts: MultisigCreateAccountsV2,
    args: MultisigCreateArgsV2,
    program_id: Option<Pubkey>,
) -> Instruction {
    Instruction {
        accounts: accounts.to_account_metas(Some(false)),
        data: MultisigCreateDataV2 { args }.data(),
        program_id: program_id.unwrap_or(squads_multisig_program::ID),
    }
}
