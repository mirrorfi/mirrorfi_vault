// Program ID constant for Kamino Lending
pub const KAMINO_LENDING_PROGRAM_ID: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";

// Import all the CPI modules
pub mod init_obligation;
pub mod init_obligation_farms_for_reserve;
pub mod refresh_reserve;
pub mod refresh_obligation;
pub mod deposit_reserve_liquidity_and_obligation_collateral_v2;
pub mod borrow_obligation_liquidity_v2;

// Re-export all module contents for easier usage
pub use init_obligation::*;
pub use init_obligation_farms_for_reserve::*;
pub use refresh_reserve::*;
pub use refresh_obligation::*;
pub use deposit_reserve_liquidity_and_obligation_collateral_v2::*;
pub use borrow_obligation_liquidity_v2::*;