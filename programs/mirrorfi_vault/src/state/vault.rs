// use anchor_lang::prelude::*;
// use derivative::Derivative;

// #[derive(InitSpace, Derivative, PartialEq)]
// #[derivative(Debug)]
// #[account(zero_copy(unsafe))]
// #[repr(C)]
// pub struct Vault {
//     /// Unique Address of the vault
//     pub pubkey: Pubkey,
//     /// Manager of the Vault
//     pub manager: Pubkey,

//     // To Add:
//     // - Fee Rate
// }

// impl Size for Protocol {
//     const SIZE: usize = 152 + 8;
// }
// const_assert_eq!(
//     VaultProtocol::SIZE,
//     std::mem::size_of::<VaultProtocol>() + 8
// );

// impl Default for Protocol {
//     fn default() -> Self {
//         // TO CHANGE:
//         Self {
//             creator: Pubkey::default(),
//             owner: Pubkey::default(),
//             created_at: 0,
//             updated_at: 0,
//             protocol_fee_authority: Pubkey::default(),
//             padding: [0; 4],
//             protocol_fee_rate: 0,
//             is_initialized: false,
//             freeze: false,
//             version: 0,
//             bump: 0,
//         }
//     }
// }

// impl Vault {
//     // TO CHANGE:
//     pub fn init(&mut self, params: InitProtocolParams) -> Result<()> {
//         *self = Self::default();
//         self.creator = params.creator;
//         self.owner = params.owner;
//         self.is_initialized = true;
//         self.version = 1;
//         self.bump = params.bump;
//         self.protocol_fee_authority = params.protocol_fee_authority;
//         self.protocol_fee_rate = params.protocol_fee_rate;
//         self.created_at = Clock::get()?.unix_timestamp;
//         self.updated_at = Clock::get()?.unix_timestamp;
//         Ok(())
//     }
// }

// pub struct InitVaultParams {
//     // TO CHANGE:
//     pub bump: u8,
//     pub creator: Pubkey,
//     pub owner: Pubkey,
//     pub protocol_fee_authority: Pubkey,
//     pub protocol_fee_rate: u32,
// }