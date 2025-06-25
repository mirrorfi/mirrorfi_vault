use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::{invoke, invoke_signed};

/// Get the Discriminator of a Program Function
/// This calculates the standard Anchor instruction discriminator (first 8 bytes of SHA-256 hash)
pub fn get_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    discriminator
}

/// Creates instruction data with discriminator and serialized args
pub fn create_instruction_data<T: AnchorSerialize>(
    instruction_name: &str,
    args: &T
) -> Result<Vec<u8>> {
    let mut instruction_data = Vec::new();
    
    // Add instruction discriminator
    let discriminator = get_discriminator(instruction_name);
    instruction_data.extend_from_slice(&discriminator);
    
    // Add serialized args
    args.serialize(&mut instruction_data)?;
    
    Ok(instruction_data)
}

/// Creates a CPI instruction with the given program ID, accounts, and instruction data
pub fn create_instruction(
    program_id: Pubkey,
    accounts: Vec<AccountMeta>,
    data: Vec<u8>
) -> Instruction {
    Instruction {
        program_id,
        accounts,
        data,
    }
}

/// Executes a CPI call with no signing PDA
pub fn execute_cpi(
    instruction: Instruction,
    account_infos: &[AccountInfo],
) -> Result<()> {
    invoke(
        &instruction,
        account_infos,
    ).map_err(Into::into)
}

/// Executes a CPI call with a signing PDA
pub fn execute_cpi_with_signer(
    instruction: Instruction,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    invoke_signed(
        &instruction,
        account_infos,
        signer_seeds,
    ).map_err(Into::into)
}
