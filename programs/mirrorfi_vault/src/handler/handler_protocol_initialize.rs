use anchor_lang::prelude::*;
use crate::state::protocol::{Protocol, InitProtocolParams};
use crate::utils::seeds::{PROTOCOL, PROTOCOL_AUTHORITY, PROTOCOL_FEE_AUTHORITY};
use crate::error::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitProtocolArgs {
    pub id: u64,
    pub protocol_fee_rate: u64,
}

#[derive(Accounts)]
#[instruction(args: InitProtocolArgs)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        space = Protocol::SIZE,
        seeds = [PROTOCOL.as_ref(), creator.key().as_ref(), &args.id.to_le_bytes()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,
    
    #[account(
        seeds = [PROTOCOL_AUTHORITY.as_ref(), protocol.key().as_ref()],
        bump,
    )]
    /// CHECK: Protocol authority PDA, will be used for various protocol-level permissions
    pub protocol_authority: UncheckedAccount<'info>,
    
    #[account(
        seeds = [PROTOCOL_FEE_AUTHORITY.as_ref(), protocol.key().as_ref()],
        bump,
    )]
    /// CHECK: Protocol fee authority PDA, will be used for fee collection
    pub protocol_fee_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<InitializeProtocol>, args: InitProtocolArgs) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    
    // Initialize the protocol with provided parameters
    let init_params = InitProtocolParams {
        bump: *ctx.bumps.get("protocol").unwrap(),
        creator: ctx.accounts.creator.key(),
        owner: ctx.accounts.creator.key(), // Initially set owner to creator
        protocol_fee_authority: ctx.accounts.protocol_fee_authority.key(),
        protocol_fee_rate: args.protocol_fee_rate,
    };
    
    protocol.init(init_params)?;
    
    msg!("Protocol initialized with ID: {}", args.id);
    
    Ok(())
}
