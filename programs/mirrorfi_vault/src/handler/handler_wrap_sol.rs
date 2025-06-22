use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{sync_native, Mint, SyncNative, TokenAccount},
};

use crate::{
    Errors::InsufficientSolToWrap,
    utils::constants::WSOL_TOKEN_MINT,
};


#[inline(never)]
pub fn handle(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_ata = &ctx.accounts.user_ata;

    require_gte!(user.lamports(), amount, InsufficientSolToWrap);

    // TRANSFER SOL to WSOL ATA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: user.to_account_info(),
            to: user_ata.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    // SYNC SOL AS WSOL
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative{
            account: user_ata.to_account_info(),
        },
    );
    sync_native(cpi_ctx)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct WrapSol<'info> {
    #[account(mut, signer)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(address = WSOL_TOKEN_MINT)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub rent: Sysvar<'info, Rent>,
}