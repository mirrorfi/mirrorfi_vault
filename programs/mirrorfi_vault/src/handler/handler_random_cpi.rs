use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, instruction::Instruction, program::invoke_signed};
use crate::utils::cpi::get_discriminator;

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

pub fn handler(ctx: Context<RandomCpi>) -> Result<()> {
    msg!("Executing CPI to plutonian_initialize instruction");

    // Get the discriminator for the plutonian_initialize instruction
    // Note: We could also use the hardcoded value from the IDL: [143, 18, 128, 64, 106, 123, 124, 122]
    let plutonian_initialize_discriminator = get_discriminator("plutonian_initialize");

    // Create the instruction data - for this instruction, we only need the discriminator
    // since it doesn't take any arguments
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&plutonian_initialize_discriminator);

    // Create account metas for the CPI call
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
    ];

    // Create the instruction
    let ix = Instruction {
        program_id: ctx.accounts.program.key(),
        accounts: account_metas,
        data: instruction_data,
    };

    // Execute the CPI call
    // Using invoke_signed without seeds here since we are not signing with a PDA
    // If we needed to sign with a PDA, we would pass the seeds to invoke_signed
    invoke_signed(
        &ix,
        &[
            ctx.accounts.actor.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.protocol.to_account_info(),
            ctx.accounts.plutonian.to_account_info(),
            ctx.accounts.plutonian_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.event_authority.to_account_info(),
            ctx.accounts.program.to_account_info(),
        ],
        &[], // No signer seeds, as we're using a keypair signer
    )?;

    msg!("Successfully executed plutonian_initialize CPI call");
    Ok(())
}