use anchor_lang::prelude::*;
use crate::cpi::get_discriminator;

// Kamino Lending Program ID - Replace this with the actual program ID
pub const KAMINO_LENDING_PROGRAM_ID: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";

use super::*;

/// Structure for init_obligation args
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitObligationArgs {
    pub tag: u8,
    pub id: u8,
}

/// The accounts required by Kamino's init_obligation instruction
#[derive(Clone)]
pub struct InitObligationAccounts<'info> {
    pub obligation_owner: AccountInfo<'info>,
    pub fee_payer: AccountInfo<'info>,
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub seed1_account: AccountInfo<'info>,
    pub seed2_account: AccountInfo<'info>,
    pub owner_user_metadata: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

/// Manual implementation for the InitObligationAccounts struct to be used in a CPI call
impl<'info> ToAccountMetas for InitObligationAccounts<'info> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let mut account_metas = Vec::new();
        
        account_metas.push(AccountMeta::new_readonly(self.obligation_owner.key(), true));  // Always a signer
        account_metas.push(AccountMeta::new(self.fee_payer.key(), true));  // Mutable and a signer
        account_metas.push(AccountMeta::new(self.obligation.key(), false));  // Mutable, not a signer
        account_metas.push(AccountMeta::new_readonly(self.lending_market.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.seed1_account.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.seed2_account.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.owner_user_metadata.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.rent.key(), false));  // Not mutable, not signer
        account_metas.push(AccountMeta::new_readonly(self.system_program.key(), false));  // Not mutable, not signer
        
        account_metas
    }
}

/// Initialize an obligation in the Kamino lending protocol
///
/// # Arguments
///
/// * `accounts` - The accounts needed for the instruction
/// * `program_id` - The Kamino lending program ID
/// * `args` - Arguments for the instruction
/// * `signer_seeds` - Seeds for PDAs that need to sign
///
/// # Returns
///
/// * `Result<()>` - Result indicating success or failure
pub fn execute<'info>(
    accounts: InitObligationAccounts<'info>,
    program_id: AccountInfo<'info>,
    args: InitObligationArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    // Instruction data (will be serialized and sent in the CPI call)
    let mut instruction_data = Vec::new();
    
    // Add instruction discriminator for init_obligation in Kamino
    // This is the first 8 bytes of sha256("global:initObligation")
    let init_obligation_discriminator = get_discriminator("initObligation");
    instruction_data.extend_from_slice(init_obligation_discriminator);
    
    // Serialize and add the args
    args.serialize(&mut instruction_data)?;
    
    // Create the instruction
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: program_id.key(),
        accounts: accounts.to_account_metas(None),
        data: instruction_data,
    };
    
    // Execute the instruction via CPI
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            accounts.obligation_owner,
            accounts.fee_payer,
            accounts.obligation,
            accounts.lending_market,
            accounts.seed1_account,
            accounts.seed2_account,
            accounts.owner_user_metadata,
            accounts.rent,
            accounts.system_program,
        ],
        signer_seeds,
    ).map_err(Into::into)
}
