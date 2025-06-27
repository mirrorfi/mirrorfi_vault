use anchor_lang::prelude::*;
use derivative::Derivative;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Vault {
    /// Name of the vault (32 bytes)
    #[derivative(Debug = "ignore")]
    pub name: [u8; 32],
    /// Unique Address of the vault
    pub pubkey: Pubkey,
    /// Manager of the Vault
    pub manager: Pubkey,
    /// Protocol associated with this vault
    pub protocol: Pubkey,
    /// Vault authority (PDA)
    pub vault_authority: Pubkey,
    /// Vault fee authority
    pub vault_fee_authority: Pubkey,
    /// Protocol fee authority
    pub protocol_fee_authority: Pubkey,
    /// Mint address of the Vault's accepted deposit token
    pub deposit_token_mint: Pubkey,
    /// Mint address of the Vault Share Position tokens
    pub share_token_mint: Pubkey,
    /// Timestamp when the vault was created
    pub created_at: i64,
    /// Timestamp of the last update
    pub updated_at: i64,
    /// Timestamp of last NAV update
    pub last_nav_update_at: i64,
    
    /// Manager fee rate, 1000 = 1% (decimal = 3)
    pub manager_fee_rate: u64,
    /// Last calculated Net Asset Value
    pub last_nav: u64,
    /// Total amount deposited into the vault
    pub total_deposit_amount: u64,
    /// Total amount withdrawn from the vault
    pub total_withdraw_amount: u64,
    /// Total number of deposit transactions
    pub total_deposit_count: u64,
    /// Total number of withdraw transactions
    pub total_withdraw_count: u64,
    /// Total yield generated
    pub total_yield: u64,
    /// Total fees paid to the manager
    pub total_manager_fee: u64,
    /// Total fees paid to the protocol
    pub total_protocol_fee: u64,
    
    /// Is the vault initialized
    pub is_initialized: bool,
    // Is the vault frozen (no deposits/withdrawals)
    pub freeze: bool,
    /// Version of the vault
    pub version: u8,
    pub bump: u8,
    /// Integration flags for different protocols
    pub is_kamino: bool,
    pub is_meteora: bool,
    pub is_orca: bool,
    pub is_drift: bool,
    /// Reserved space for future protocol integrations
    #[derivative(Debug = "ignore")]
    pub reserved_protocols: [bool; 16],

    /// Padding for future use
    #[derivative(Debug = "ignore")]
    pub padding: [u64; 192],
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            name: [0; 32],
            pubkey: Pubkey::default(),
            manager: Pubkey::default(),
            protocol: Pubkey::default(),
            vault_authority: Pubkey::default(),
            vault_fee_authority: Pubkey::default(),
            protocol_fee_authority: Pubkey::default(),
            deposit_token_mint: Pubkey::default(),
            share_token_mint: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            last_nav_update_at: 0,
            last_nav: 0,
            manager_fee_rate: 0,
            total_deposit_amount: 0,
            total_withdraw_amount: 0,
            total_deposit_count: 0,
            total_withdraw_count: 0,
            total_yield: 0,
            total_manager_fee: 0,
            total_protocol_fee: 0,
            is_initialized: false,
            version: 0,
            bump: 0,
            freeze: false,
            is_kamino: false,
            is_meteora: false,
            is_orca: false,
            is_drift: false,
            reserved_protocols: [false; 16],
            padding: [0; 192],
        }
    }
}

impl Vault {
    pub const SIZE: usize = std::mem::size_of::<Vault>() + 8;
    
    pub fn init(&mut self, params: InitVaultParams) -> Result<()> {
        *self = Self::default();
        
        // Copy name bytes if provided
        if !params.name.is_empty() {
            let mut name_bytes = [0u8; 32];
            let len = std::cmp::min(params.name.len(), 32);
            name_bytes[..len].copy_from_slice(&params.name[..len]);
            self.name = name_bytes;
        }
        
        self.pubkey = params.pubkey;
        self.manager = params.manager;
        self.protocol = params.protocol;
        self.vault_authority = params.vault_authority;
        self.vault_fee_authority = params.vault_fee_authority;
        self.protocol_fee_authority = params.protocol_fee_authority;
        self.deposit_token_mint = params.deposit_token_mint;
        self.share_token_mint = params.share_token_mint;
        self.manager_fee_rate = params.manager_fee_rate;
        
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        
        // Set timestamps
        let now = Clock::get()?.unix_timestamp;
        self.created_at = now;
        self.updated_at = now;
        self.last_nav_update_at = now;
        
        Ok(())
    }
}

pub struct InitVaultParams {
    /// Name of the vault
    pub name: Vec<u8>,
    /// Pubkey of the vault account
    pub pubkey: Pubkey,
    /// Manager of the vault
    pub manager: Pubkey,
    /// Protocol account
    pub protocol: Pubkey,
    /// Vault authority
    pub vault_authority: Pubkey,
    /// Vault fee authority
    pub vault_fee_authority: Pubkey,
    /// Protocol fee authority
    pub protocol_fee_authority: Pubkey,
    /// Deposit token mint
    pub deposit_token_mint: Pubkey,
    /// Share token mint
    pub share_token_mint: Pubkey,
    /// Manager fee rate (1000 = 1%, decimal = 3)
    pub manager_fee_rate: u64,
    /// Bump seed for PDA
    pub bump: u8,
}