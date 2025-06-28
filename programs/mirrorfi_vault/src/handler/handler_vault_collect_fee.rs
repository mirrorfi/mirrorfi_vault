use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, transfer, Transfer};
use crate::state::vault::Vault;
use crate::state::protocol::Protocol;
use crate::utils::seeds;
use crate::error::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CollectFeeArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct CollectVaultFee<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,
    
    /// The protocol account this vault belongs to
    #[account(
        constraint = !protocol.freeze @ ErrorCode::ProtocolFrozen
    )]
    pub protocol: Account<'info, Protocol>,
    
    #[account(
        has_one = manager @ ErrorCode::UnauthorizedAccess,
        has_one = vault_authority @ ErrorCode::InvalidVaultAuthority,
        has_one = vault_fee_authority @ ErrorCode::InvalidVaultFeeAuthority,
        has_one = deposit_token_mint @ ErrorCode::InvalidDepositTokenMint,
        has_one = protocol @ ErrorCode::InvalidProtocol,
        constraint = !vault.freeze @ ErrorCode::VaultFrozen,
    )]
    pub vault: Account<'info, Vault>,
    
    /// The deposit token mint
    /// CHECK: Validated in the has_one constraint above
    pub deposit_token_mint: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [seeds::VAULT_AUTHORITY.as_ref(), vault.key().as_ref()],
        bump,
    )]
    /// CHECK: This is the vault authority PDA
    pub vault_authority: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [seeds::VAULT_FEE_AUTHORITY.as_ref(), vault.key().as_ref()],
        bump,
    )]
    /// CHECK: This is the vault fee authority PDA
    pub vault_fee_authority: AccountInfo<'info>,
    
    #[account(
        mut,
        token_interface::mint = deposit_token_mint,
        token_interface::authority = vault_authority,
    )]
    /// Vault token account holding the deposit tokens
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    #[account(
        mut,
        token_interface::mint = deposit_token_mint,
        token_interface::authority = vault_fee_authority,
    )]
    /// Fee recipient token account controlled by the vault fee authority
    pub fee_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// SPL Token interface program
    pub token_program: Interface<'info, TokenInterface>,
    
    /// Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handle(ctx: Context<CollectVaultFee>, args: CollectFeeArgs) -> Result<()> {
    // Ensure the amount is greater than zero
    require!(args.amount > 0, ErrorCode::InvalidAmount);
    
    // Transfer tokens from vault token account to the fee token account
    let seeds = &[
        seeds::VAULT_AUTHORITY.as_ref(),
        ctx.accounts.vault.key().as_ref(),
        &[*ctx.bumps.get("vault_authority").unwrap()],
    ];
    let signer_seeds = &[&seeds[..]];
    
    // Create the transfer instruction
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.fee_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    
    // Execute the transfer
    transfer(transfer_ctx, args.amount)?;
    
    // Update the vault state
    let vault = &mut ctx.accounts.vault;
    vault.total_manager_fee = vault.total_manager_fee.checked_add(args.amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    vault.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Collected {} vault fees successfully", args.amount);
    
    Ok(())
}
