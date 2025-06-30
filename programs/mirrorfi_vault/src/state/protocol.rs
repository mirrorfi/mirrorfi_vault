use anchor_lang::prelude::*;
use derivative::Derivative;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Protocol {
    /// Address of protocol's creator
    pub creator: Pubkey,
    /// Update authority of protocol state
    pub owner: Pubkey,
    /// Creation timestamp of protocol
    pub created_at: i64,
    /// Last updated timestamp of the protocol
    pub updated_at: i64,
     /// Fee Authority of the protocol
    pub protocol_fee_authority: Pubkey,
    /// Extra Space
    #[derivative(Debug = "ignore")]
    pub padding: [u64; 64],
    /// The Fee Rate of the protocol, 1000 = 1% (decimal = 3)
    #[derivative(Default(value="0u64"))] // 0%
    pub protocol_fee_rate: u64, // 0%
    /// Protocol Initialization State
    pub is_initialized: bool,
    /// Protocol Freeze State
    pub freeze: bool,
    /// Protocol Version
    pub version: u8,
    pub bump: u8,
}

impl Default for Protocol {
    fn default() -> Self {
        Self {
            creator: Pubkey::default(),
            owner: Pubkey::default(),
            created_at: 0,
            updated_at: 0,
            protocol_fee_authority: Pubkey::default(),
            padding: [0; 4],
            protocol_fee_rate: 0,
            is_initialized: false,
            freeze: false,
            version: 0,
            bump: 0,
        }
    }
}

impl Protocol {
    pub const SIZE: usize = std::mem::size_of::<Protocol>() + 8;

    pub fn init(&mut self, params: InitProtocolParams) -> Result<()> {
        *self = Self::default();
        self.creator = params.creator;
        self.owner = params.owner;
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol_fee_authority = params.protocol_fee_authority;
        self.protocol_fee_rate = params.protocol_fee_rate;
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    /// Freeze the protocol to prevent further operations
    pub fn freeze(&mut self) -> Result<()> {
        self.freeze = true;
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    /// Unfreeze the protocol to allow operations
    pub fn unfreeze(&mut self) -> Result<()> {
        self.freeze = false;
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

pub struct InitProtocolParams {
    pub bump: u8,
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub protocol_fee_authority: Pubkey,
    pub protocol_fee_rate: u32,
}