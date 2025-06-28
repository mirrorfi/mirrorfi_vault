use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // General errors
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    // Sol wrapping errors
    #[msg("Insufficient SOL to wrap")]
    InsufficientSolToWrap,
    
    #[msg("Insufficient WSOL to unwrap")]
    InsufficientWSolToUnwrap,
    
    // Protocol related errors
    #[msg("Protocol is frozen")]
    ProtocolFrozen,
    
    #[msg("Protocol is already frozen")]
    AlreadyFrozen,
    
    #[msg("Protocol is already unfrozen")]
    AlreadyUnfrozen,
    
    // Vault related errors
    #[msg("Vault is frozen")]
    VaultFrozen,
    
    #[msg("Invalid vault authority")]
    InvalidVaultAuthority,
    
    #[msg("Invalid vault fee authority")]
    InvalidVaultFeeAuthority,
    
    #[msg("Invalid deposit token mint")]
    InvalidDepositTokenMint,
    
    #[msg("Invalid share token mint")]
    InvalidShareTokenMint,
    
    #[msg("Invalid protocol reference")]
    InvalidProtocol,
    
    // Token related errors
    #[msg("Invalid authority")]
    InvalidAuthority,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Invalid destination account")]
    InvalidDestinationAccount,
    
    #[msg("Operation not allowed")]
    OperationNotAllowed,
}
