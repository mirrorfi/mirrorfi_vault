use anchor_lang::prelude::*;
use crate::state::protocol::Protocol;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UnfreezeProtocol<'info> {
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        has_one = owner @ ErrorCode::UnauthorizedAccess,
    )]
    pub protocol: Account<'info, Protocol>,
}

pub fn handle(ctx: Context<UnfreezeProtocol>) -> Result<()> {
    // Get a mutable reference to the protocol account
    let protocol = &mut ctx.accounts.protocol;
    
    // Check if protocol is already unfrozen
    require!(protocol.freeze, ErrorCode::AlreadyUnfrozen);
    
    // Unfreeze the protocol
    protocol.freeze = false;
    
    // Update the timestamp
    protocol.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Protocol unfrozen successfully");
    
    Ok(())
}
