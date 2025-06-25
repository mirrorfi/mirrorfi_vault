use anchor_lang::prelude::*;
use crate::cpi::kamino;
use crate::cpi::kamino::init_obligation::KAMINO_LENDING_PROGRAM_ID;

pub fn handle(ctx: Context<KaminoInitObligation>, args: KaminoInitObligationArgs) -> Result<()> {
    // Create the instruction args for the CPI call
    let init_args = kamino::init_obligation::InitObligationArgs {
        tag: args.tag,
        id: args.id,
    };
    
    // Create the accounts structure for the CPI call
    let accounts = kamino::init_obligation::InitObligationAccounts {
        obligation_owner: ctx.accounts.obligation_owner.to_account_info(),
        fee_payer: ctx.accounts.fee_payer.to_account_info(),
        obligation: ctx.accounts.obligation.to_account_info(),
        lending_market: ctx.accounts.lending_market.to_account_info(),
        seed1_account: ctx.accounts.seed1_account.to_account_info(),
        seed2_account: ctx.accounts.seed2_account.to_account_info(),
        owner_user_metadata: ctx.accounts.owner_user_metadata.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    // Make the CPI call to init_obligation
    kamino::init_obligation::execute(
        accounts,
        ctx.accounts.kamino_program.to_account_info(),
        init_args,
        &[], // No signer seeds needed since the obligation_owner is signing directly
    )?;

    msg!("Successfully initialized obligation in Kamino lending protocol");
    Ok(())
}

#[derive(Accounts)]
pub struct KaminoInitObligation<'info> {
    /// The signer of the obligation (user's wallet)
    pub obligation_owner: Signer<'info>,

    /// The account that will pay the transaction fee
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    /// CHECK: The obligation account to be created on Kamino
    pub obligation: UncheckedAccount<'info>,

    /// CHECK: The lending market account from Kamino lending program
    pub lending_market: UncheckedAccount<'info>,

    /// CHECK: Seed account (can be a Mint address depending on the tag) validated by Kamino
    pub seed1_account: UncheckedAccount<'info>,
    
    /// CHECK: Seed account (can be a Mint address depending on the tag) validated by Kamino
    pub seed2_account: UncheckedAccount<'info>,

    /// CHECK: Owner's user metadata account from Kamino validated by the Kamino program
    pub owner_user_metadata: UncheckedAccount<'info>,

    /// CHECK: The Kamino lending program ID that we'll be making a CPI call to
    #[account(address = KAMINO_LENDING_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub kamino_program: UncheckedAccount<'info>,

    /// Required by Kamino's init_obligation function
    pub rent: Sysvar<'info, Rent>,
    
    /// Required by Kamino's init_obligation function
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct KaminoInitObligationArgs {
    pub tag: u8,
    pub id: u8,
}
