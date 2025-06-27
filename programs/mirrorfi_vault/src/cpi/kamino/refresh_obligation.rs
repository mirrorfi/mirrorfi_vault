use anchor_lang::prelude::*;
use crate::utils::cpi::*;
use super::KAMINO_LENDING_PROGRAM_ID;

/// The accounts required by Kamino's refresh_obligation instruction
#[derive(Clone)]
pub struct RefreshObligationAccounts<'info> {
    /// The obligation to refresh
    pub obligation: AccountInfo<'info>,
    
    /// All reserve accounts that the obligation has deposits or borrows in
    /// This is a variable length array of accounts that must be provided
    pub reserves: Vec<AccountInfo<'info>>,
}

/// Manual implementation for the RefreshObligationAccounts struct to be used in a CPI call
impl<'info> ToAccountMetas for RefreshObligationAccounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        // Add obligation as mutable but not signer
        account_metas.push(AccountMeta::new(self.obligation.key(), false));
        
        // Add all reserves as read-only and not signers
        for reserve in &self.reserves {
            account_metas.push(AccountMeta::new_readonly(reserve.key(), false));
        }
        
        account_metas
    }
}

/// Refresh an obligation in the Kamino lending protocol (update debt and collateral values)
///
/// # Arguments
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
pub fn execute<'info>(
    accounts: RefreshObligationAccounts<'info>,
    program_id: AccountInfo<'info>,
) -> Result<()> {
    // Define the accounts needed for the instruction using our ToAccountMetas implementation
    let account_metas = accounts.to_account_metas(None);

    // This instruction doesn't have any arguments, so we create empty args struct
    struct EmptyArgs {}
    
    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("refreshObligation", &EmptyArgs {})?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // Get account infos in the same order as the account metas
    let mut account_infos = Vec::with_capacity(accounts.reserves.len() + 2); // +2 for obligation and program_id
    account_infos.push(accounts.obligation.to_account_info());
    
    // Add all reserves
    for reserve in &accounts.reserves {
        account_infos.push(reserve.clone());
    }
    
    // Add the program id
    account_infos.push(program_id);
    
    // Execute the instruction using our helper function
    execute_cpi(ix, &account_infos)
}
