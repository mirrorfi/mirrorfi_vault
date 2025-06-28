use anchor_lang::prelude::*;
use crate::state::vault::{Vault, InitVaultParams};
use crate::state::protocol::Protocol;
use crate::utils::seeds::{VAULT, VAULT_AUTHORITY, VAULT_FEE_AUTHORITY, PROTOCOL_FEE_AUTHORITY};
use crate::error::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitVaultArgs {
    pub id: u64,
    pub name: Vec<u8>,
    pub manager_fee_rate: u64,
}

#[derive(Accounts)]
#[instruction(args: InitVaultArgs)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    /// The protocol account this vault belongs to
    #[account(
        constraint = !protocol.freeze @ ErrorCode::ProtocolFrozen
    )]
    pub protocol: Account<'info, Protocol>,
    
    #[account(
        init,
        payer = creator,
        space = Vault::SIZE,
        seeds = [VAULT.as_ref(), creator.key().as_ref(), &args.id.to_le_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        seeds = [VAULT_AUTHORITY.as_ref(), vault.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault authority PDA, will be used for vault operations
    pub vault_authority: UncheckedAccount<'info>,
    
    #[account(
        seeds = [VAULT_FEE_AUTHORITY.as_ref(), vault.key().as_ref()],
        bump,
    )]
    /// CHECK: Vault fee authority PDA, will be used for fee collection
    pub vault_fee_authority: UncheckedAccount<'info>,
    
    /// CHECK: The protocol fee authority account
    #[account(
        seeds = [PROTOCOL_FEE_AUTHORITY.as_ref(), protocol.key().as_ref()],
        bump,
    )]
    pub protocol_fee_authority: UncheckedAccount<'info>,
    
    /// Mint address of the deposit token
    /// CHECK: This is the deposit token mint, will be verified elsewhere
    pub deposit_token_mint: UncheckedAccount<'info>,
    
    /// Mint address of the share token (LP token)
    /// CHECK: This is the share token mint, will be verified elsewhere
    pub share_token_mint: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<InitializeVault>, args: InitVaultArgs) -> Result<()> {
    // Get a mutable reference to the vault account
    let vault = &mut ctx.accounts.vault;
    
    // Initialize the vault with provided parameters
    let init_params = InitVaultParams {
        name: args.name,
        pubkey: ctx.accounts.vault.key(),
        manager: ctx.accounts.creator.key(),
        protocol: ctx.accounts.protocol.key(),
        vault_authority: ctx.accounts.vault_authority.key(),
        vault_fee_authority: ctx.accounts.vault_fee_authority.key(),
        protocol_fee_authority: ctx.accounts.protocol_fee_authority.key(),
        deposit_token_mint: ctx.accounts.deposit_token_mint.key(),
        share_token_mint: ctx.accounts.share_token_mint.key(),
        manager_fee_rate: args.manager_fee_rate,
        bump: *ctx.bumps.get("vault").unwrap(),
    };
    
    vault.init(init_params)?;
    
    msg!("Vault initialized with ID: {}", args.id);
    
    Ok(())
}
