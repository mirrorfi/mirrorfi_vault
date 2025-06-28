mod state;
mod utils;
mod handler;
mod cpi;

use crate::handler::*;
use anchor_lang::prelude::*;

declare_id!("5NK8X7nuDaVB8ZhLGsbsUSXiFszbXjwSUp2FZN1vHA55");

pub mod error;
use error::ErrorCode;

#[program]
pub mod mirrorfi_vault {
    use super::*;

    // === SOL Wrapping/Unwrapping ===
    pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
        handler_wrap_sol::handle(ctx, amount)
    }
    
    pub fn unwrap_sol(ctx: Context<UnwrapSol>, amount: u64) -> Result<()> {
        handler_unwrap_sol::handle(ctx, amount)
    }
    
    // === Protocol Management ===
    pub fn protocol_initialize(ctx: Context<InitializeProtocol>, args: InitProtocolArgs) -> Result<()> {
        handler_protocol_initialize::handle(ctx, args)
    }
    
    pub fn protocol_freeze(ctx: Context<FreezeProtocol>) -> Result<()> {
        handler_protocol_freeze::handle(ctx)
    }
    
    pub fn protocol_unfreeze(ctx: Context<UnfreezeProtocol>) -> Result<()> {
        handler_protocol_unfreeze::handle(ctx)
    }
    
    // === Vault Management ===
    pub fn vault_initialize(ctx: Context<InitializeVault>, args: InitVaultArgs) -> Result<()> {
        handler_vault_initialize::handle(ctx, args)
    }
    
    pub fn vault_freeze(ctx: Context<FreezeVault>) -> Result<()> {
        handler_vault_freeze::handle(ctx)
    }
    
    pub fn vault_unfreeze(ctx: Context<UnfreezeVault>) -> Result<()> {
        handler_vault_unfreeze::handle(ctx)
    }
    
    pub fn vault_collect_fee(ctx: Context<CollectVaultFee>, args: CollectFeeArgs) -> Result<()> {
        handler_vault_collect_fee::handle(ctx, args)
    }
    
    // === Kamino Lending Integration ===
    /// This function initializes a new obligation in the Kamino lending protocol
    pub fn kamino_init_obligation(ctx: Context<KaminoInitObligation>, args: KaminoInitObligationArgs) -> Result<()> {
        handler_kamino_init_obligation::handle(ctx, args)
    }

    // === Testing & Development ===
    pub fn random_cpi(ctx: Context<RandomCpi>) -> Result<()> {
        handler_random_cpi::handle(ctx)
    }
}

// Use the dedicated error module instead
#[error_code]
pub enum Errors {
    #[msg("insufficient sol to wrap")]
    InsufficientSolToWrap,
    #[msg("insufficient wsol to unwrap")]
    InsufficientWSolToUnwrap,
}