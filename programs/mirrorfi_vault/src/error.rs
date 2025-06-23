use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient SOL to wrap")]
    InsufficientSolToWrap,
    
    #[msg("Insufficient WSOL to unwrap")]
    InsufficientWSolToUnwrap,
    
    #[msg("Protocol is frozen")]
    ProtocolFrozen,
    
    #[msg("Invalid authority")]
    InvalidAuthority,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Invalid destination account")]
    InvalidDestinationAccount,
    
    #[msg("Operation not allowed")]
    OperationNotAllowed,
}
