use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UnfreezeVault<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,
    
    #[account(
        mut,
        has_one = manager @ ErrorCode::UnauthorizedAccess,
    )]
    pub vault: Account<'info, Vault>,
}

pub fn handle(ctx: Context<UnfreezeVault>) -> Result<()> {
    // Get a mutable reference to the vault account
    let vault = &mut ctx.accounts.vault;
    
    // Check if vault is already unfrozen
    require!(vault.freeze, ErrorCode::AlreadyUnfrozen);
    
    // Unfreeze the vault
    vault.freeze = false;
    
    // Update the timestamp
    vault.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Vault unfrozen successfully");
    
    Ok(())
}
