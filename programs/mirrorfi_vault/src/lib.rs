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

    pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
        handler_wrap_sol::handle(ctx, amount)
    }
    
    pub fn unwrap_sol(ctx: Context<UnwrapSol>, amount: u64) -> Result<()> {
        handler_unwrap_sol::handle(ctx, amount)
    }
    
    /// This function initializes a new obligation in the Kamino lending protocol
    pub fn kamino_init_obligation(ctx: Context<KaminoInitObligation>, args: KaminoInitObligationArgs) -> Result<()> {
        handler_kamino_init_obligation::handle(ctx, args)
    }

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