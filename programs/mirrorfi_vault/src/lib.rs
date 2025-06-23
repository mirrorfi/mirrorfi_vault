mod state;
mod utils;
mod handler;

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

    // pub fn unwrap_all_sol(ctx: Context<UnwrapAllSol>) -> Result<()> {
    //     handler_unwrap_all_sol::handle(ctx)
    // }
}

// Use the dedicated error module instead
#[error_code]
pub enum Errors {
    #[msg("insufficient sol to wrap")]
    InsufficientSolToWrap,
    #[msg("insufficient wsol to unwrap")]
    InsufficientWSolToUnwrap,
}