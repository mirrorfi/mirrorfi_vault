use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::AccountMeta;
use crate::utils::cpi::*;

/// The Pluto Leverage Program ID
pub const PLUTO_LEVERAGE_PROGRAM_ID: &str = "DNcR7b5ZpU7X4nTa62sTmroyvsSa52d66hunbCaMUjq2";

/// The RandomCpi accounts for calling plutonian_initialize
#[derive(Accounts)]
pub struct RandomCpi<'info> {
    /// Signer of the transaction
    #[account(mut)]
    pub actor: Signer<'info>,

    /// User account that will own the Plutonian
    /// CHECK: This is passed to the external program
    pub user: UncheckedAccount<'info>,

    /// Protocol account of the Pluto Leverage protocol
    /// CHECK: This is passed to the external program
    pub protocol: UncheckedAccount<'info>,

    /// The Plutonian account to initialize
    /// CHECK: This is passed to the external program and will be initialized by the CPI
    #[account(mut)]
    pub plutonian: UncheckedAccount<'info>,

    /// Plutonian Authority PDA from the Pluto Leverage program
    /// CHECK: This is a PDA created by the external program
    pub plutonian_authority: UncheckedAccount<'info>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Event Authority for the Pluto Leverage program
    /// CHECK: This is a PDA created by the external program for event emission
    pub event_authority: UncheckedAccount<'info>,

    /// Program to invoke
    /// CHECK: This is the Pluto Leverage program we're calling
    #[account(address = PLUTO_LEVERAGE_PROGRAM_ID.parse::<Pubkey>().unwrap())]
    pub program: UncheckedAccount<'info>,
}

pub fn handle(ctx: Context<RandomCpi>) -> Result<()> {
    msg!("Executing CPI to plutonian_initialize instruction");

    // Create account metas for the CPI call using our structured approach
    let account_metas = vec![
        // actor - writable and signer
        AccountMeta::new(ctx.accounts.actor.key(), true),
        // user - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.user.key(), false),
        // protocol - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.protocol.key(), false),
        // plutonian - writable, not signer
        AccountMeta::new(ctx.accounts.plutonian.key(), false),
        // plutonian_authority - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.plutonian_authority.key(), false),
        // system_program - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        // event_authority - not writable, not signer
        AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
        // program - not writable, not signer (the program account itself needs to be included)
        AccountMeta::new_readonly(ctx.accounts.program.key(), false),
    ];
    
    // This instruction doesn't have any arguments, so we create empty args struct
    struct EmptyArgs {}
    
    // Create instruction data using our helper function
    let instruction_data = create_instruction_data("plutonian_initialize", &EmptyArgs {})?;
    
    // Create the instruction using our helper function
    let ix = create_instruction(
        ctx.accounts.program.key(),
        account_metas, 
        instruction_data
    );

    msg!("Invoking instruction to Pluto Leverage program: {}", ctx.accounts.program.key());

    // Gather account infos in the same order as account_metas
    let account_infos = &[
        ctx.accounts.actor.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.protocol.to_account_info(),
        ctx.accounts.plutonian.to_account_info(),
        ctx.accounts.plutonian_authority.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.event_authority.to_account_info(),
        ctx.accounts.program.to_account_info(),
    ];
    
    // Execute the instruction using our helper function
    // No signer seeds needed as we're using a keypair signer
    execute_cpi(ix, account_infos)?;

    msg!("Successfully executed plutonian_initialize CPI call");
    Ok(())
}