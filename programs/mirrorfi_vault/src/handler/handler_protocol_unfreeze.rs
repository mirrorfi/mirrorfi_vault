use anchor_lang::prelude::*;
use crate::state::protocol::Protocol;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UnfreezeProtocol<'info> {
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        has_one = owner,
    )]
    pub protocol: Account<'info, Protocol>,
}

pub fn handle(ctx: Context<UnfreezeProtocol>) -> Result<()> {
    // Get a mutable reference to the protocol account
    let protocol = &mut ctx.accounts.protocol;
    
    // Unfreeze the protocol using the method on Protocol
    protocol.unfreeze()?;

    msg!("Protocol unfrozen by: {}", self.owner);

    Ok(())
}
