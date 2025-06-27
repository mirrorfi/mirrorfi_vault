use anchor_lang::prelude::*;
use crate::utils::cpi::*;
use super::KAMINO_LENDING_PROGRAM_ID;

/// Structure for init_obligation_farms_for_reserve args
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct InitObligationFarmsForReserveArgs {
    pub farms_count: u64,
}

/// The accounts required by Kamino's init_obligation_farms_for_reserve instruction
#[derive(Clone)]
pub struct InitObligationFarmsForReserveAccounts<'info> {
    /// Owner of the obligation
    pub owner: AccountInfo<'info>,
    
    /// The obligation account
    pub obligation: AccountInfo<'info>,
    
    /// The lending market account
    pub lending_market: AccountInfo<'info>,
    
    /// The reserve for which farms are being initialized
    pub reserve: AccountInfo<'info>,
    
    /// The obligation's farm state account
    pub obligation_farms_for_reserve: AccountInfo<'info>,
    
    /// The system program
    pub system_program: AccountInfo<'info>,
    
    /// The rent sysvar
    pub rent: AccountInfo<'info>,
}

/// Manual implementation for the InitObligationFarmsForReserveAccounts struct to be used in a CPI call
impl<'info> ToAccountMetas for InitObligationFarmsForReserveAccounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        account_metas.push(AccountMeta::new_readonly(self.owner.key(), true));  // Owner is signer
        account_metas.push(AccountMeta::new(self.obligation.key(), false));  // Mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.lending_market.key(), false));  // Not mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.reserve.key(), false));  // Not mutable, not a signer
        account_metas.push(AccountMeta::new(self.obligation_farms_for_reserve.key(), false));  // Mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.system_program.key(), false));  // Not mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.rent.key(), false));  // Not mutable, not a signer
        
        account_metas
    }
}

/// Initialize farm obligations for a specific reserve in the Kamino lending protocol
///
/// # Arguments
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
/// * `args` - Arguments for the instruction
pub fn execute<'info>(
    accounts: InitObligationFarmsForReserveAccounts<'info>,
    program_id: AccountInfo<'info>,
    args: InitObligationFarmsForReserveArgs,
) -> Result<()> {
    // Define the accounts needed for the instruction using our ToAccountMetas implementation
    let account_metas = accounts.to_account_metas(None);

    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("initObligationFarmsForReserve", &args)?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // Get account infos in the same order as the account metas
    let account_infos = &[
        accounts.owner.to_account_info(),
        accounts.obligation.to_account_info(),
        accounts.lending_market.to_account_info(),
        accounts.reserve.to_account_info(),
        accounts.obligation_farms_for_reserve.to_account_info(),
        accounts.system_program.to_account_info(),
        accounts.rent.to_account_info(),
        program_id,
    ];

    // Execute the instruction using our helper function
    execute_cpi(ix, account_infos)
}
