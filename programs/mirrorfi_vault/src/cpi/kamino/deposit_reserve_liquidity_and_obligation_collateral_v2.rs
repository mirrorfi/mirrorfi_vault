use anchor_lang::prelude::*;
use crate::utils::cpi::*;
use super::KAMINO_LENDING_PROGRAM_ID;

/// Structure for deposit_reserve_liquidity_and_obligation_collateral_v2 args
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct DepositReserveLiquidityAndObligationCollateralV2Args {
    pub liquidity_amount: u64,
}

/// The accounts required by Kamino's deposit_reserve_liquidity_and_obligation_collateral_v2 instruction
#[derive(Clone)]
pub struct DepositReserveLiquidityAndObligationCollateralV2Accounts<'info> {
    /// Source liquidity token account
    pub source_liquidity: AccountInfo<'info>,
    
    /// Destination collateral token account
    pub destination_collateral: AccountInfo<'info>,
    
    /// The reserve account being deposited to
    pub reserve: AccountInfo<'info>,
    
    /// Reserve liquidity supply SPL token account
    pub reserve_liquidity_supply: AccountInfo<'info>,
    
    /// Reserve collateral SPL token mint
    pub reserve_collateral_mint: AccountInfo<'info>,
    
    /// Lending market account
    pub lending_market: AccountInfo<'info>,
    
    /// Lending market authority (PDA)
    pub lending_market_authority: AccountInfo<'info>,
    
    /// Obligation account to deposit collateral to
    pub obligation: AccountInfo<'info>,
    
    /// Obligation owner
    pub owner: AccountInfo<'info>,
    
    /// Pyth oracle price account
    pub pyth_oracle_price: AccountInfo<'info>,
    
    /// Switchboard oracle price account
    pub switchboard_oracle_price: AccountInfo<'info>,
    
    /// Scope oracle price account
    pub scope_price: AccountInfo<'info>,
    
    /// Optional FLT denomination token mint
    pub flt_denomination_token_mint: Option<AccountInfo<'info>>,
    
    /// Token program ID
    pub token_program: AccountInfo<'info>,
}

/// Manual implementation for the DepositReserveLiquidityAndObligationCollateralV2Accounts struct to be used in a CPI call
impl<'info> ToAccountMetas for DepositReserveLiquidityAndObligationCollateralV2Accounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        account_metas.push(AccountMeta::new(self.source_liquidity.key(), false));
        account_metas.push(AccountMeta::new(self.destination_collateral.key(), false));
        account_metas.push(AccountMeta::new(self.reserve.key(), false));
        account_metas.push(AccountMeta::new(self.reserve_liquidity_supply.key(), false));
        account_metas.push(AccountMeta::new(self.reserve_collateral_mint.key(), false));
        account_metas.push(AccountMeta::new_readonly(self.lending_market.key(), false));
        account_metas.push(AccountMeta::new_readonly(self.lending_market_authority.key(), false));
        account_metas.push(AccountMeta::new(self.obligation.key(), false));
        account_metas.push(AccountMeta::new_readonly(self.owner.key(), true)); // Owner is signer
        account_metas.push(AccountMeta::new_readonly(self.pyth_oracle_price.key(), false));
        account_metas.push(AccountMeta::new_readonly(self.switchboard_oracle_price.key(), false));
        account_metas.push(AccountMeta::new_readonly(self.scope_price.key(), false));
        
        // Optional FLT denomination token mint
        if let Some(flt_mint) = &self.flt_denomination_token_mint {
            account_metas.push(AccountMeta::new_readonly(flt_mint.key(), false));
        }
        
        account_metas.push(AccountMeta::new_readonly(self.token_program.key(), false));
        
        account_metas
    }
}

/// Deposit reserve liquidity and obligation collateral in the Kamino lending protocol (V2)
///
/// # Arguments
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
/// * `args` - Arguments for the instruction
pub fn execute<'info>(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'info>,
    program_id: AccountInfo<'info>,
    args: DepositReserveLiquidityAndObligationCollateralV2Args,
) -> Result<()> {
    // Define the accounts needed for the instruction using our ToAccountMetas implementation
    let account_metas = accounts.to_account_metas(None);

    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("depositReserveLiquidityAndObligationCollateralV2", &args)?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        program_id.key(),
        account_metas,
        instruction_data
    );
    
    // Get account infos in the same order as the account metas
    let mut account_infos = Vec::with_capacity(15); // Capacity for all potential accounts
    
    account_infos.push(accounts.source_liquidity.to_account_info());
    account_infos.push(accounts.destination_collateral.to_account_info());
    account_infos.push(accounts.reserve.to_account_info());
    account_infos.push(accounts.reserve_liquidity_supply.to_account_info());
    account_infos.push(accounts.reserve_collateral_mint.to_account_info());
    account_infos.push(accounts.lending_market.to_account_info());
    account_infos.push(accounts.lending_market_authority.to_account_info());
    account_infos.push(accounts.obligation.to_account_info());
    account_infos.push(accounts.owner.to_account_info());
    account_infos.push(accounts.pyth_oracle_price.to_account_info());
    account_infos.push(accounts.switchboard_oracle_price.to_account_info());
    account_infos.push(accounts.scope_price.to_account_info());
    
    // Optional FLT denomination token mint
    if let Some(flt_mint) = &accounts.flt_denomination_token_mint {
        account_infos.push(flt_mint.to_account_info());
    }
    
    account_infos.push(accounts.token_program.to_account_info());
    account_infos.push(program_id);

    // Execute the instruction using our helper function
    execute_cpi(ix, &account_infos)
}
