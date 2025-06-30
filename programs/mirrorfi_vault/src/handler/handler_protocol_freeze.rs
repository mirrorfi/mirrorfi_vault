use anchor_lang::prelude::*;
use crate::state::protocol::Protocol;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct FreezeProtocol<'info> {
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        has_one = owner,
    )]
    pub protocol: Account<'info, Protocol>,
}

pub fn handle(ctx: Context<FreezeProtocol>) -> Result<()> {
    // Get a mutable reference to the protocol account
    let protocol = &mut ctx.accounts.protocol;
    
    // Freeze the protocol using the method on Protocol
    protocol.freeze()?;

    msg!("Protocol frozen by: {}", self.owner);

    Ok(())
}
