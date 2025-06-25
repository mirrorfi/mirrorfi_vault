use anchor_lang::prelude::*;
use crate::utils::cpi::*;

// Kamino Lending Program ID - Replace this with the actual program ID
pub const KAMINO_LENDING_PROGRAM_ID: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";

use super::*;

/// Structure for init_obligation args
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitObligationArgs {
    pub tag: u8,
    pub id: u8,
}

/// The accounts required by Kamino's init_obligation instruction
#[derive(Clone)]
pub struct InitObligationAccounts<'info> {
    pub obligation_owner: AccountInfo<'info>,
    pub fee_payer: AccountInfo<'info>,
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub seed1_account: AccountInfo<'info>,
    pub seed2_account: AccountInfo<'info>,
    pub owner_user_metadata: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

/// Manual implementation for the InitObligationAccounts struct to be used in a CPI call
impl<'info> ToAccountMetas for InitObligationAccounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        account_metas.push(AccountMeta::new_readonly(self.obligation_owner.key(), true));  // Always a signer
        account_metas.push(AccountMeta::new(self.fee_payer.key(), true));  // Mutable and a signer
        account_metas.push(AccountMeta::new(self.obligation.key(), false));  // Mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.lending_market.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.seed1_account.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.seed2_account.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.owner_user_metadata.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.rent.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.system_program.key(), false));  // Not mutable, not signer
        
        account_metas
    }
}

/// Initialize an obligation in the Kamino lending protocol
///
/// # Arguments
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
/// * `args` - Arguments for the instruction
pub fn execute<'info>(
    accounts: InitObligationAccounts<'info>,
    program_id: AccountInfo<'info>,
    args: InitObligationArgs,
) -> Result<()> {
    // Define the accounts needed for the instruction using our ToAccountMetas implementation
    let account_metas = accounts.to_account_metas(None);

    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("initObligation", &args)?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // Get account infos in the same order as the account metas
    let account_infos = &[
        accounts.obligation_owner.to_account_info(),
        accounts.fee_payer.to_account_info(),
        accounts.obligation.to_account_info(),
        accounts.lending_market.to_account_info(),
        accounts.seed1_account.to_account_info(),
        accounts.seed2_account.to_account_info(),
        accounts.owner_user_metadata.to_account_info(),
        accounts.rent.to_account_info(),
        accounts.system_program.to_account_info(),
        program_id,
    ];

    // Execute the instruction using our helper function
    execute_cpi(ix, account_infos)
}
