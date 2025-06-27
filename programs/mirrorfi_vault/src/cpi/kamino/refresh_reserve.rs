use anchor_lang::prelude::*;
use crate::utils::cpi::*;
use super::KAMINO_LENDING_PROGRAM_ID;

/// The accounts required by Kamino's refresh_reserve instruction
#[derive(Clone)]
pub struct RefreshReserveAccounts<'info> {
    /// The reserve to refresh
    pub reserve: AccountInfo<'info>,
    
    /// Oracle for the reserve asset
    pub pyth_oracle_price: AccountInfo<'info>,
    
    /// Switchboard oracle for the reserve asset
    pub switchboard_oracle_price: AccountInfo<'info>,
    
    /// Scope oracle price account
    pub scope_price: AccountInfo<'info>,
}

/// Manual implementation for the RefreshReserveAccounts struct to be used in a CPI call
impl<'info> ToAccountMetas for RefreshReserveAccounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        account_metas.push(AccountMeta::new(self.reserve.key(), false));  // Mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.pyth_oracle_price.key(), false));  // Not mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.switchboard_oracle_price.key(), false));  // Not mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.scope_price.key(), false));  // Not mutable, not a signer
        
        account_metas
    }
}

/// Refresh a reserve in the Kamino lending protocol (update interest rates and prices)
///
/// # Arguments
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
pub fn execute<'info>(
    accounts: RefreshReserveAccounts<'info>,
    program_id: AccountInfo<'info>,
) -> Result<()> {
    // Define the accounts needed for the instruction using our ToAccountMetas implementation
    let account_metas = accounts.to_account_metas(None);

    // This instruction doesn't have any arguments, so we create empty args struct
    struct EmptyArgs {}
    
    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("refreshReserve", &EmptyArgs {})?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // Get account infos in the same order as the account metas
    let account_infos = &[
        accounts.reserve.to_account_info(),
        accounts.pyth_oracle_price.to_account_info(),
        accounts.switchboard_oracle_price.to_account_info(),
        accounts.scope_price.to_account_info(),
        program_id,
    ];

    // Execute the instruction using our helper function
    execute_cpi(ix, account_infos)
}
