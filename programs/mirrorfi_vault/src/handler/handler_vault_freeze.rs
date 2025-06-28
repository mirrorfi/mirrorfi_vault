use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct FreezeVault<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,
    
    #[account(
        mut,
        has_one = manager @ ErrorCode::UnauthorizedAccess,
    )]
    pub vault: Account<'info, Vault>,
}

pub fn handle(ctx: Context<FreezeVault>) -> Result<()> {
    // Get a mutable reference to the vault account
    let vault = &mut ctx.accounts.vault;
    
    // Check if vault is already frozen
    require!(!vault.freeze, ErrorCode::AlreadyFrozen);
    
    // Freeze the vault
    vault.freeze = true;
    
    // Update the timestamp
    vault.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Vault frozen successfully");
    
    Ok(())
}
